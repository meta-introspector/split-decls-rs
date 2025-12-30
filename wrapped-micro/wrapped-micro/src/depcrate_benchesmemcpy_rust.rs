// Generated macro for memcpy_rust (function)
macro_rules! Depcrate_benchesmemcpy_rust {
() => {
// Module: crate::benches
// Provides: {"memcpy_rust"}
// Dependencies: {}
fn memcpy_rust (n : usize) { let v1 = vec ! [1u8 ; n] ; let mut v2 = vec ! [0u8 ; n] ; let now = Instant :: now () ; for _i in 0 .. NR_RUNS { let src : & [u8] = black_box (& v1 [0 ..]) ; let dst : & mut [u8] = black_box (& mut v2 [0 ..]) ; unsafe { memcpy (dst . as_mut_ptr () as * mut c_void , src . as_ptr () as * mut c_void , n ,) ; } } hermit_bench_output :: log_benchmark_data_with_group (& format ! ("(rust) block size {n}") , "MByte/s" , ((NR_RUNS * n) >> 20) as f64 / now . elapsed () . as_secs_f64 () , "Memcpy speed" ,) ; }
};
}
