// Generated macro for impl_546 (impl)
macro_rules! Depcrate_diffimpl_546 {
() => {
// Module: crate::diff
// Provides: {"impl_546"}
// Dependencies: {}
impl < 'a > Binding for DiffHunk < 'a > { type Raw = * const raw :: git_diff_hunk ; unsafe fn from_raw (raw : * const raw :: git_diff_hunk) -> DiffHunk < 'a > { DiffHunk { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_hunk { self . raw } }
};
}
