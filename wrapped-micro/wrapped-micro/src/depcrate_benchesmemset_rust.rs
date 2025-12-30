// Generated macro for memset_rust (function)
macro_rules! Depcrate_benchesmemset_rust {
() => {
// Module: crate::benches
// Provides: {"memset_rust"}
// Dependencies: {}
fn memset_rust (n : usize) { let mut v1 = vec ! [0u8 ; n] ; let now = Instant :: now () ; for _i in 0 .. NR_RUNS { let dst : & mut [u8] = black_box (& mut v1 [0 ..]) ; let val = black_box (27) ; unsafe { memset (dst . as_mut_ptr () as * mut c_void , val , n) ; } } hermit_bench_output :: log_benchmark_data_with_group (& format ! ("(rust) block size {n}") , "MByte/s" , ((NR_RUNS * n) >> 20) as f64 / now . elapsed () . as_secs_f64 () , "Memset speed" ,) ; }
};
}
