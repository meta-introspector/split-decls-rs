// Generated macro for impl_769 (impl)
macro_rules! Depcrate_odbimpl_769 {
() => {
// Module: crate::odb
// Provides: {"impl_769"}
// Dependencies: {}
impl < 'repo > Binding for OdbReader < 'repo > { type Raw = * mut raw :: git_odb_stream ; unsafe fn from_raw (raw : * mut raw :: git_odb_stream) -> OdbReader < 'repo > { OdbReader { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_stream { self . raw } }
};
}
