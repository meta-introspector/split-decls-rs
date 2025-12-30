// Generated macro for impl_543 (impl)
macro_rules! Depcrate_diffimpl_543 {
() => {
// Module: crate::diff
// Provides: {"impl_543"}
// Dependencies: {}
impl < 'a > Binding for DiffLine < 'a > { type Raw = * const raw :: git_diff_line ; unsafe fn from_raw (raw : * const raw :: git_diff_line) -> DiffLine < 'a > { DiffLine { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_line { self . raw } }
};
}
