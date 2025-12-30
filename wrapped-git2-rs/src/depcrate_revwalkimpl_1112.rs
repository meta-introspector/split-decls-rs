// Generated macro for impl_1112 (impl)
macro_rules! Depcrate_revwalkimpl_1112 {
() => {
// Module: crate::revwalk
// Provides: {"impl_1112"}
// Dependencies: {}
impl < 'repo > Binding for Revwalk < 'repo > { type Raw = * mut raw :: git_revwalk ; unsafe fn from_raw (raw : * mut raw :: git_revwalk) -> Revwalk < 'repo > { Revwalk { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_revwalk { self . raw } }
};
}
