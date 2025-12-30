// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_signatureimpl_1128 {
() => {
// Module: crate::signature
// Provides: {"impl_1128"}
// Dependencies: {}
impl < 'a > Binding for Signature < 'a > { type Raw = * mut raw :: git_signature ; unsafe fn from_raw (raw : * mut raw :: git_signature) -> Signature < 'a > { Signature { raw , _marker : marker :: PhantomData , owned : true , } } fn raw (& self) -> * mut raw :: git_signature { self . raw } }
};
}
