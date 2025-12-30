// Generated macro for impl_760 (impl)
macro_rules! Depcrate_odbimpl_760 {
() => {
// Module: crate::odb
// Provides: {"impl_760"}
// Dependencies: {}
impl < 'repo > Binding for Odb < 'repo > { type Raw = * mut raw :: git_odb ; unsafe fn from_raw (raw : * mut raw :: git_odb) -> Odb < 'repo > { Odb { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb { self . raw } }
};
}
