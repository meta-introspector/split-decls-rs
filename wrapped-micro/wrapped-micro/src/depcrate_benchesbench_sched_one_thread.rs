// Generated macro for bench_sched_one_thread (function)
macro_rules! Depcrate_benchesbench_sched_one_thread {
() => {
// Module: crate::benches
// Provides: {"bench_sched_one_thread"}
// Dependencies: {}
pub fn bench_sched_one_thread () -> Result < () , () > { let n = 1000000 ; thread :: yield_now () ; thread :: yield_now () ; let _ = get_timestamp () ; let start = get_timestamp () ; for _ in 0 .. n { thread :: yield_now () ; } let ticks = get_timestamp () - start ; hermit_bench_output :: log_benchmark_data_with_group ("1 thread" , "ticks" , ticks as f64 / n as f64 , "Scheduling time" ,) ; Ok (()) }
};
}
