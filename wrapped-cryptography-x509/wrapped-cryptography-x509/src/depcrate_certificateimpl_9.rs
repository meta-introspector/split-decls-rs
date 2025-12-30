// Generated macro for impl_9 (impl)
macro_rules! Depcrate_certificateimpl_9 {
() => {
// Module: crate::certificate
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > TbsCertificate < 'a > { pub fn extensions (& self) -> Result < Extensions < 'a > , DuplicateExtensionsError > { Extensions :: from_raw_extensions (self . raw_extensions . as_ref ()) } }
};
}
