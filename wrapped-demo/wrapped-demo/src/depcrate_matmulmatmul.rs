// Generated macro for matmul (function)
macro_rules! Depcrate_matmulmatmul {
() => {
// Module: crate::matmul
// Provides: {"matmul"}
// Dependencies: {}
pub fn matmul () { eprintln ! () ; eprintln ! ("Matrix multiplication") ; if SIZE <= 1024 { timed_matmul (SIZE , seq_matmul , "seq row-major") ; } let seq = if SIZE <= 2048 { timed_matmul (SIZE , seq_matmulz , "seq z-order") } else { 0 } ; let par = timed_matmul (SIZE , matmulz , "par z-order") ; timed_matmul (SIZE , matmul_strassen , "par strassen") ; let speedup = seq as f64 / par as f64 ; eprintln ! ("speedup: {speedup:.2}x") ; }
};
}
