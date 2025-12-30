// Generated macro for sincos (function)
macro_rules! Depcrate_mathsincos {
() => {
// Module: crate::math
// Provides: {"sincos"}
// Dependencies: {}
# [linkage = "weak_odr"] # [unsafe (no_mangle)] pub extern "C" fn sincos (x : f64 , s : & mut f64 , c : & mut f64) { (* s , * c) = libm :: sincos (x) ; }
};
}
