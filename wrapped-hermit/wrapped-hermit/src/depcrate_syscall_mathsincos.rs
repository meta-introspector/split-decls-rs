// Generated macro for sincos (function)
macro_rules! Depcrate_syscall_mathsincos {
() => {
// Module: crate::syscall::math
// Provides: {"sincos"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sincos (x : f64 , s : & mut f64 , c : & mut f64) { (* s , * c) = libm :: sincos (x) ; }
};
}
