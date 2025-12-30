// Generated macro for impl_3 (impl)
macro_rules! Depcrate_accessimpl_3 {
() => {
// Module: crate::access
// Provides: {"impl_3"}
// Dependencies: {}
impl ScreenCaptureAccess { # [doc = " If current app not in list, will open window."] # [doc = " Return the same result as preflight."] # [inline] pub fn request (& self) -> bool { unsafe { CGRequestScreenCaptureAccess () } } # [doc = " Return `true` if has access"] # [inline] pub fn preflight (& self) -> bool { unsafe { CGPreflightScreenCaptureAccess () } } }
};
}
