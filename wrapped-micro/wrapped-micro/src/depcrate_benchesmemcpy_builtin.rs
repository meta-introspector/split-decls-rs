// Generated macro for memcpy_builtin (function)
macro_rules! Depcrate_benchesmemcpy_builtin {
() => {
// Module: crate::benches
// Provides: {"memcpy_builtin"}
// Dependencies: {}
fn memcpy_builtin (n : usize) { let v1 = vec ! [1u8 ; n] ; let mut v2 = vec ! [0u8 ; n] ; let now = Instant :: now () ; for _i in 0 .. NR_RUNS { let src : & [u8] = black_box (& v1) ; let dst : & mut [u8] = black_box (& mut v2) ; dst . copy_from_slice (src) ; } hermit_bench_output :: log_benchmark_data_with_group (& format ! ("(built_in) block size {n}") , "MByte/s" , (NR_RUNS * n) as f64 / (1024.0 * 1024.0 * now . elapsed () . as_secs_f64 ()) , "Memcpy speed" ,) ; }
};
}
