// Generated macro for impl_16 (impl)
macro_rules! Depcrate_impls_slice_mutimpl_16 {
() => {
// Module: crate::impls::slice_mut
// Provides: {"impl_16"}
// Dependencies: {}
impl Error for SliceWriteError { fn kind (& self) -> ErrorKind { match self { SliceWriteError :: Full => ErrorKind :: WriteZero , } } }
};
}
