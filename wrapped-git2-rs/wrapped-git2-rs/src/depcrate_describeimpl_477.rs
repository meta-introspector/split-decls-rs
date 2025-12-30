// Generated macro for impl_477 (impl)
macro_rules! Depcrate_describeimpl_477 {
() => {
// Module: crate::describe
// Provides: {"impl_477"}
// Dependencies: {}
impl < 'repo > Binding for Describe < 'repo > { type Raw = * mut raw :: git_describe_result ; unsafe fn from_raw (raw : * mut raw :: git_describe_result) -> Describe < 'repo > { Describe { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_describe_result { self . raw } }
};
}
