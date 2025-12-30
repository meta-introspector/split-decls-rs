// Generated macro for impl_907 (impl)
macro_rules! Depcrate_rebaseimpl_907 {
() => {
// Module: crate::rebase
// Provides: {"impl_907"}
// Dependencies: {}
impl < 'repo > Binding for Rebase < 'repo > { type Raw = * mut raw :: git_rebase ; unsafe fn from_raw (raw : * mut raw :: git_rebase) -> Rebase < 'repo > { Rebase { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_rebase { self . raw } }
};
}
