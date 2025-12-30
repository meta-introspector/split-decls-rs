// Generated macro for impl_938 (impl)
macro_rules! Depcrate_referenceimpl_938 {
() => {
// Module: crate::reference
// Provides: {"impl_938"}
// Dependencies: {}
impl < 'repo > Binding for References < 'repo > { type Raw = * mut raw :: git_reference_iterator ; unsafe fn from_raw (raw : * mut raw :: git_reference_iterator) -> References < 'repo > { References { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_reference_iterator { self . raw } }
};
}
