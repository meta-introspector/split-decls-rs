// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { run_benchmark_group (| group | { group . register_benchmark ("nbody_5k" , | | { let mut nbody = nbody :: init (5000) ; | | { for _ in 0 .. 10 { nbody = nbody :: compute_forces (nbody) ; } nbody } }) ; }) ; }
};
}
