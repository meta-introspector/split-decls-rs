// Generated macro for impl_102 (impl)
macro_rules! Depcrate_errorimpl_102 {
() => {
// Module: crate::error
// Provides: {"impl_102"}
// Dependencies: {}
impl ser :: Error for Error { # [cold] fn custom < T : Display > (msg : T) -> Error { make_error (msg . to_string ()) } }
};
}
