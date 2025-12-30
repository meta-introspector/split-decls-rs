// Generated macro for nanf (function)
macro_rules! Depcrate_mathnanf {
() => {
// Module: crate::math
// Provides: {"nanf"}
// Dependencies: {}
# [doc = " nanf"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `arg` has to point to a valid [`CStr`]."] # [doc = ""] # [doc = " [`CStr`]: core::ffi::CStr"] # [linkage = "weak_odr"] # [unsafe (no_mangle)] pub unsafe extern "C" fn nanf (arg : * const core :: ffi :: c_char) -> f32 { let _arg = arg ; f32 :: NAN }
};
}
