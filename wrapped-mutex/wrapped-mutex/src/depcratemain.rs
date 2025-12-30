// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let available_parallelism = thread :: available_parallelism () . unwrap () . get () ; let mut i = 1 ; while i <= available_parallelism { mutex_stress_test (black_box (i)) ; i *= 2 ; } }
};
}
