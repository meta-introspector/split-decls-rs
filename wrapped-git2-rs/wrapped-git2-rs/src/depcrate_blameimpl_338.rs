// Generated macro for impl_338 (impl)
macro_rules! Depcrate_blameimpl_338 {
() => {
// Module: crate::blame
// Provides: {"impl_338"}
// Dependencies: {}
impl < 'repo > Binding for Blame < 'repo > { type Raw = * mut raw :: git_blame ; unsafe fn from_raw (raw : * mut raw :: git_blame) -> Blame < 'repo > { Blame { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_blame { self . raw } }
};
}
