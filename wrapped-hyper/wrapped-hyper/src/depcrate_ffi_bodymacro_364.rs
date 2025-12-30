// Generated macro for macro_364 (macro)
macro_rules! Depcrate_ffi_bodymacro_364 {
() => {
// Module: crate::ffi::body
// Provides: {"macro_364"}
// Dependencies: {}
ffi_fn ! { # [doc = " Set userdata on this body, which will be passed to callback functions."] fn hyper_body_set_userdata (body : * mut hyper_body , userdata : * mut c_void) { let b = non_null ! (& mut * body ?= ()) ; b . 0 . as_ffi_mut () . userdata = userdata ; } }
};
}
