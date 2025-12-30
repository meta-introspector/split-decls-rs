// Generated macro for sincosf (function)
macro_rules! Depcrate_syscall_mathsincosf {
() => {
// Module: crate::syscall::math
// Provides: {"sincosf"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sincosf (x : f32 , s : & mut f32 , c : & mut f32) { (* s , * c) = libm :: sincosf (x) ; }
};
}
