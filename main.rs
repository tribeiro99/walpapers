use rand::Rng;
use std::io;
use std::f64::consts::PI;

fn main() {
    let mut repetir = String::new();
    let mut rng = rand::thread_rng();

    loop {
        println!("Digite o número de pontos: ");
        let mut num_pontos = String::new();
        io::stdin().read_line(&mut num_pontos).expect("Falha ao ler entrada");
        
        let num_pontos: usize = match num_pontos.trim().parse() {
            Ok(n) if n > 0 => n,
            _ => {
                println!("Entrada inválida! O número deve ser positivo.");
                continue;
            }
        };

        let mut pontos_dentro = 0;

        for _ in 0..num_pontos {
            let x: f64 = rng.gen_range(-1.0..1.0);
            let y: f64 = rng.gen_range(-1.0..1.0);

            if x * x + y * y <= 1.0 {
                pontos_dentro += 1;
            }
        }

        let pi_aprox = (pontos_dentro as f64 / num_pontos as f64) * 4.0;
        let erro = (pi_aprox - PI).abs();

        println!("Pontos dentro do círculo: {}", pontos_dentro);
        println!("Aproximação de π: {:.6}", pi_aprox);
        println!("Erro absoluto: {:.6}", erro);
        
        println!("\nQueres fazer outra simulação? (s/n): ");
        repetir.clear();
        io::stdin().read_line(&mut repetir).expect("Falha ao ler entrada");
        let repetir = repetir.trim().to_lowercase();
        
        if repetir != "s" {
            break;
        }
    }
    println!("Adeus!");
}

