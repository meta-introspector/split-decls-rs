// Generated macro for bench_mem (function)
macro_rules! Depcrate_benchesbench_mem {
() => {
// Module: crate::benches
// Provides: {"bench_mem"}
// Dependencies: {}
pub fn bench_mem () -> Result < () , () > { memcpy_builtin (black_box (4096)) ; memcpy_builtin (black_box (1048576)) ; memcpy_builtin (black_box (16 * 1048576)) ; memset_builtin (black_box (4096)) ; memset_builtin (black_box (1048576)) ; memset_builtin (black_box (16 * 1048576)) ; memcpy_rust (black_box (4096)) ; memcpy_rust (black_box (1048576)) ; memcpy_rust (black_box (16 * 1048576)) ; memset_rust (black_box (4096)) ; memset_rust (black_box (1048576)) ; memset_rust (black_box (16 * 1048576)) ; Ok (()) }
};
}
