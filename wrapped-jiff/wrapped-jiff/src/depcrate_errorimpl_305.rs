// Generated macro for impl_305 (impl)
macro_rules! Depcrate_errorimpl_305 {
() => {
// Module: crate::error
// Provides: {"impl_305"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl IntoError for alloc :: string :: String { # [inline (always)] fn into_error (self) -> Error { Error :: adhoc_from_string (self) } }
};
}
