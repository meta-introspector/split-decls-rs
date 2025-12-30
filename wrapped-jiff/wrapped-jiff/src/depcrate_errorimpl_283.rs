// Generated macro for impl_283 (impl)
macro_rules! Depcrate_errorimpl_283 {
() => {
// Module: crate::error
// Provides: {"impl_283"}
// Dependencies: {}
impl From < ErrorKind > for Error { fn from (kind : ErrorKind) -> Error { # [cfg (feature = "alloc")] { Error { inner : Some (Arc :: new (ErrorInner { kind , cause : None })) } } # [cfg (not (feature = "alloc"))] { Error { inner : Some (Arc :: new (ErrorInner { kind })) } } } }
};
}
