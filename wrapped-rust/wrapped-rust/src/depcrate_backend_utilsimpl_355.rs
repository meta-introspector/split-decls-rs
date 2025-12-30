// Generated macro for impl_355 (impl)
macro_rules! Depcrate_backend_utilsimpl_355 {
() => {
// Module: crate::backend::utils
// Provides: {"impl_355"}
// Dependencies: {}
impl BytesOrPyBytes < '_ > { pub (crate) fn as_bytes (& self) -> & [u8] { match self { BytesOrPyBytes :: Bytes (v) => v , BytesOrPyBytes :: PyBytes (v) => v . as_bytes () , } } }
};
}
