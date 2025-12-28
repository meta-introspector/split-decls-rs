macro_rules! sin_pi {
    () => {
        fn sin_pi (mut x : f32) -> f32 { let mut y : f64 ; let mut n : isize ; x = 2.0 * (x * 0.5 - floorf (x * 0.5)) ; n = (x * 4.0) as isize ; n = div ! (n + 1 , 2) ; y = (x as f64) - (n as f64) * 0.5 ; y *= 3.14159265358979323846 ; match n { 1 => k_cosf (y) , 2 => k_sinf (- y) , 3 => - k_cosf (y) , _ => k_sinf (y) , } }
    };
}

sin_pi!()