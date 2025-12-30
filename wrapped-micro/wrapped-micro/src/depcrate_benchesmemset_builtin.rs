// Generated macro for memset_builtin (function)
macro_rules! Depcrate_benchesmemset_builtin {
() => {
// Module: crate::benches
// Provides: {"memset_builtin"}
// Dependencies: {}
fn memset_builtin (n : usize) { let mut v1 = vec ! [0u8 ; n] ; let now = Instant :: now () ; for _i in 0 .. NR_RUNS { let dst : & mut [u8] = black_box (& mut v1) ; let val : u8 = black_box (27) ; for b in dst { * b = val ; } } hermit_bench_output :: log_benchmark_data_with_group (& format ! ("(built_in) block size {n}") , "MByte/s" , ((NR_RUNS * n) >> 20) as f64 / now . elapsed () . as_secs_f64 () , "Memset speed" ,) ; }
};
}
