// Generated macro for impl_304 (impl)
macro_rules! Depcrate_errorimpl_304 {
() => {
// Module: crate::error
// Provides: {"impl_304"}
// Dependencies: {}
impl IntoError for & 'static str { # [inline (always)] fn into_error (self) -> Error { Error :: adhoc_from_static_str (self) } }
};
}
