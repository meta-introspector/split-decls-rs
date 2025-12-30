// Generated macro for impl_775 (impl)
macro_rules! Depcrate_odbimpl_775 {
() => {
// Module: crate::odb
// Provides: {"impl_775"}
// Dependencies: {}
impl < 'repo > Binding for OdbWriter < 'repo > { type Raw = * mut raw :: git_odb_stream ; unsafe fn from_raw (raw : * mut raw :: git_odb_stream) -> OdbWriter < 'repo > { OdbWriter { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_stream { self . raw } }
};
}
