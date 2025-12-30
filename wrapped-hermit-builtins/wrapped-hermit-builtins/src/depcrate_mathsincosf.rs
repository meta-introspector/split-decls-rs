// Generated macro for sincosf (function)
macro_rules! Depcrate_mathsincosf {
() => {
// Module: crate::math
// Provides: {"sincosf"}
// Dependencies: {}
# [linkage = "weak_odr"] # [unsafe (no_mangle)] pub extern "C" fn sincosf (x : f32 , s : & mut f32 , c : & mut f32) { (* s , * c) = libm :: sincosf (x) ; }
};
}
