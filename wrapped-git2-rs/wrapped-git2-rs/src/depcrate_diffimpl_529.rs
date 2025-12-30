// Generated macro for impl_529 (impl)
macro_rules! Depcrate_diffimpl_529 {
() => {
// Module: crate::diff
// Provides: {"impl_529"}
// Dependencies: {}
impl < 'a > Binding for DiffDelta < 'a > { type Raw = * mut raw :: git_diff_delta ; unsafe fn from_raw (raw : * mut raw :: git_diff_delta) -> DiffDelta < 'a > { DiffDelta { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_diff_delta { self . raw } }
};
}
