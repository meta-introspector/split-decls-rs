// Generated macro for impl_651 (impl)
macro_rules! Depcrate_mempackimpl_651 {
() => {
// Module: crate::mempack
// Provides: {"impl_651"}
// Dependencies: {}
impl < 'odb > Binding for Mempack < 'odb > { type Raw = * mut raw :: git_odb_backend ; unsafe fn from_raw (raw : * mut raw :: git_odb_backend) -> Mempack < 'odb > { Mempack { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_backend { self . raw } }
};
}
