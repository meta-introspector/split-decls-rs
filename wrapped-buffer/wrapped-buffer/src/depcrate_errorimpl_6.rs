// Generated macro for impl_6 (impl)
macro_rules! Depcrate_errorimpl_6 {
() => {
// Module: crate::error
// Provides: {"impl_6"}
// Dependencies: {}
impl Error { pub (crate) fn unsupported (expected : & 'static str , actual : & 'static str) -> Self { Error (ErrorKind :: Unsupported { actual , expected }) } pub (crate) fn outside_container (method : & 'static str) -> Self { Error (ErrorKind :: OutsideContainer { method }) } # [doc = "\n    The given value is invalid.\n    "] pub (crate) fn invalid_value (reason : & 'static str) -> Self { Error (ErrorKind :: InvalidValue { reason }) } # [track_caller] pub (crate) fn no_alloc (method : & 'static str) -> Self { # [cfg (all (debug_assertions , not (feature = "no_debug_assertions") , not (test)))] { panic ! ("attempt to allocate for {} would fail; add the `alloc` feature of `sval_buffer` or the depdendent `sval_*` library to support allocation. This call will error instead of panicking in release builds. Add the `feature = no_debug_assertions` feature of `sval_buffer` if this error is expected." , method) ; } # [cfg (not (all (debug_assertions , not (feature = "no_debug_assertions") , not (test))))] { Error (ErrorKind :: NoAlloc { method }) } } }
};
}
