// Generated macro for impl_555 (impl)
macro_rules! Depcrate_diffimpl_555 {
() => {
// Module: crate::diff
// Provides: {"impl_555"}
// Dependencies: {}
impl < 'a > Binding for DiffBinaryFile < 'a > { type Raw = * const raw :: git_diff_binary_file ; unsafe fn from_raw (raw : * const raw :: git_diff_binary_file) -> DiffBinaryFile < 'a > { DiffBinaryFile { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_binary_file { self . raw } }
};
}
