// Generated macro for impl_433 (impl)
macro_rules! Depcrate_common_if_modified_sinceimpl_433 {
() => {
// Module: crate::common::if_modified_since
// Provides: {"impl_433"}
// Dependencies: {}
impl IfModifiedSince { # [doc = " Check if the supplied time means the resource has been modified."] pub fn is_modified (& self , last_modified : SystemTime) -> bool { self . 0 < last_modified . into () } }
};
}
