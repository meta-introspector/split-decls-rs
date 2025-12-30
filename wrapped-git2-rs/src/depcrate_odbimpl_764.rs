// Generated macro for impl_764 (impl)
macro_rules! Depcrate_odbimpl_764 {
() => {
// Module: crate::odb
// Provides: {"impl_764"}
// Dependencies: {}
impl < 'a > Binding for OdbObject < 'a > { type Raw = * mut raw :: git_odb_object ; unsafe fn from_raw (raw : * mut raw :: git_odb_object) -> OdbObject < 'a > { OdbObject { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_object { self . raw } }
};
}
