// Generated macro for impl_841 (impl)
macro_rules! Depcrate_patchimpl_841 {
() => {
// Module: crate::patch
// Provides: {"impl_841"}
// Dependencies: {}
impl < 'buffers > Binding for Patch < 'buffers > { type Raw = * mut raw :: git_patch ; unsafe fn from_raw (raw : Self :: Raw) -> Self { Patch { raw , buffers : PhantomData , } } fn raw (& self) -> Self :: Raw { self . raw } }
};
}
