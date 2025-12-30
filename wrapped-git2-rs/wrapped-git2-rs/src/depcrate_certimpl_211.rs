// Generated macro for impl_211 (impl)
macro_rules! Depcrate_certimpl_211 {
() => {
// Module: crate::cert
// Provides: {"impl_211"}
// Dependencies: {}
impl < 'a > Binding for Cert < 'a > { type Raw = * mut raw :: git_cert ; unsafe fn from_raw (raw : * mut raw :: git_cert) -> Cert < 'a > { Cert { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_cert { self . raw } }
};
}
