// Generated macro for calc_pi_multi_threaded (function)
macro_rules! Depcratecalc_pi_multi_threaded {
() => {
// Module: crate
// Provides: {"calc_pi_multi_threaded"}
// Dependencies: {}
fn calc_pi_multi_threaded (n : u64 , threads : u64) -> f64 { let mut handles = vec ! [] ; let mut pi = 0.0 ; let step = 1.0 / n as f64 ; for i in 0 .. threads { let start = i * n / threads ; let end = (i + 1) * n / threads ; let handle = thread :: spawn (move | | { let mut pi = 0.0 ; for i in start .. end { let x = (i as f64 + 0.5) * step ; pi += 4.0 / (1.0 + x * x) ; } pi }) ; handles . push (handle) ; } for handle in handles { pi += handle . join () . unwrap () ; } pi *= step ; pi }
};
}
