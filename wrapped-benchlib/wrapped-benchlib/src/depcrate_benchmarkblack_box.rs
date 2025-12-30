// Generated macro for black_box (function)
macro_rules! Depcrate_benchmarkblack_box {
() => {
// Module: crate::benchmark
// Provides: {"black_box"}
// Dependencies: {}
# [doc = " Copied from `iai`, so that it works on Rustc older than 1.66."] pub fn black_box < T > (dummy : T) -> T { unsafe { let ret = std :: ptr :: read_volatile (& dummy) ; std :: mem :: forget (dummy) ; ret } }
};
}
