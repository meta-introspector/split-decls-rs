// Generated macro for impl_532 (impl)
macro_rules! Depcrate_diffimpl_532 {
() => {
// Module: crate::diff
// Provides: {"impl_532"}
// Dependencies: {}
impl < 'a > Binding for DiffFile < 'a > { type Raw = * const raw :: git_diff_file ; unsafe fn from_raw (raw : * const raw :: git_diff_file) -> DiffFile < 'a > { DiffFile { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_file { self . raw } }
};
}
