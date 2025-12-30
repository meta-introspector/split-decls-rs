// Generated macro for bench_sched_two_threads (function)
macro_rules! Depcrate_benchesbench_sched_two_threads {
() => {
// Module: crate::benches
// Provides: {"bench_sched_two_threads"}
// Dependencies: {}
pub fn bench_sched_two_threads () -> Result < () , () > { let n = 1000000 ; let nthreads = 2 ; thread :: yield_now () ; thread :: yield_now () ; let _ = get_timestamp () ; let start = get_timestamp () ; let threads : Vec < _ > = (0 .. nthreads - 1) . map (| _ | { thread :: spawn (move | | { for _ in 0 .. n { thread :: yield_now () ; } }) }) . collect () ; for _ in 0 .. n { thread :: yield_now () ; } let ticks = get_timestamp () - start ; for t in threads { t . join () . unwrap () ; } hermit_bench_output :: log_benchmark_data_with_group ("2 threads" , "ticks" , ticks as f64 / (nthreads * n) as f64 , "Scheduling time" ,) ; Ok (()) }
};
}
