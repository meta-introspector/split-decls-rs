// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl Error { pub (crate) fn buffer (err : sval_buffer :: Error) -> Self { Error (ErrorKind :: Buffer (err)) } # [doc = "\n    The given value is invalid.\n    "] pub fn invalid_value (reason : & 'static str) -> Self { Error (ErrorKind :: InvalidValue { reason }) } # [cfg (not (feature = "alloc"))] # [track_caller] pub (crate) fn no_alloc (method : & 'static str) -> Self { # [cfg (all (debug_assertions , not (no_debug_assertions) , not (test)))] { panic ! ("attempt to allocate for {} would fail; add the `alloc` feature of `sval_nested` or the depdendent `sval_*` library to support allocation. This call will error instead of panicking in release builds. Add the `no_debug_assertions` feature of `sval_nested` if this error is expected." , method) ; } # [cfg (not (all (debug_assertions , not (no_debug_assertions) , not (test))))] { Error (ErrorKind :: NoAlloc { method }) } } }
};
}
