// Generated macro for impl_553 (impl)
macro_rules! Depcrate_diffimpl_553 {
() => {
// Module: crate::diff
// Provides: {"impl_553"}
// Dependencies: {}
impl < 'a > Binding for DiffBinary < 'a > { type Raw = * const raw :: git_diff_binary ; unsafe fn from_raw (raw : * const raw :: git_diff_binary) -> DiffBinary < 'a > { DiffBinary { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_binary { self . raw } }
};
}
