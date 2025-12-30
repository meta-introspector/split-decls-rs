// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { bench_sched_one_thread () . unwrap () ; bench_sched_two_threads () . unwrap () ; bench_syscall () . unwrap () ; bench_mem () . unwrap () ; }
};
}
