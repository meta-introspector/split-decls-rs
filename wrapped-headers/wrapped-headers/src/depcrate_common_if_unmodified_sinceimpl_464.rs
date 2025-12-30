// Generated macro for impl_464 (impl)
macro_rules! Depcrate_common_if_unmodified_sinceimpl_464 {
() => {
// Module: crate::common::if_unmodified_since
// Provides: {"impl_464"}
// Dependencies: {}
impl IfUnmodifiedSince { # [doc = " Check if the supplied time passes the precondtion."] pub fn precondition_passes (& self , last_modified : SystemTime) -> bool { self . 0 >= last_modified . into () } }
};
}
