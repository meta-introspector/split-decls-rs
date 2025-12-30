// Generated macro for impl_526 (impl)
macro_rules! Depcrate_diffimpl_526 {
() => {
// Module: crate::diff
// Provides: {"impl_526"}
// Dependencies: {}
impl < 'repo > Binding for Diff < 'repo > { type Raw = * mut raw :: git_diff ; unsafe fn from_raw (raw : * mut raw :: git_diff) -> Diff < 'repo > { Diff { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_diff { self . raw } }
};
}
