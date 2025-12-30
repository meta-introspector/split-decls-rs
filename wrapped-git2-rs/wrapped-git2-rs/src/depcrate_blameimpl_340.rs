// Generated macro for impl_340 (impl)
macro_rules! Depcrate_blameimpl_340 {
() => {
// Module: crate::blame
// Provides: {"impl_340"}
// Dependencies: {}
impl < 'blame > Binding for BlameHunk < 'blame > { type Raw = * mut raw :: git_blame_hunk ; unsafe fn from_raw (raw : * mut raw :: git_blame_hunk) -> BlameHunk < 'blame > { BlameHunk { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_blame_hunk { self . raw } }
};
}
