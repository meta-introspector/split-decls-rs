// Generated macro for impl_935 (impl)
macro_rules! Depcrate_referenceimpl_935 {
() => {
// Module: crate::reference
// Provides: {"impl_935"}
// Dependencies: {}
impl < 'repo > Binding for Reference < 'repo > { type Raw = * mut raw :: git_reference ; unsafe fn from_raw (raw : * mut raw :: git_reference) -> Reference < 'repo > { Reference { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_reference { self . raw } }
};
}
