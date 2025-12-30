// Generated macro for nan (function)
macro_rules! Depcrate_mathnan {
() => {
// Module: crate::math
// Provides: {"nan"}
// Dependencies: {}
# [doc = " nan"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `arg` has to point to a valid [`CStr`]."] # [doc = ""] # [doc = " [`CStr`]: core::ffi::CStr"] # [linkage = "weak_odr"] # [unsafe (no_mangle)] pub unsafe extern "C" fn nan (arg : * const core :: ffi :: c_char) -> f64 { let _arg = arg ; f64 :: NAN }
};
}
