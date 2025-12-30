// Generated macro for impl_6 (impl)
macro_rules! Depcrate_certificateimpl_6 {
() => {
// Module: crate::certificate
// Provides: {"impl_6"}
// Dependencies: {}
impl < 'a > Certificate < 'a > { # [doc = " Returns the certificate's issuer."] pub fn issuer (& self) -> & NameReadable < '_ > { self . tbs_cert . issuer . unwrap_read () } # [doc = " Returns the certificate's subject."] pub fn subject (& self) -> & NameReadable < '_ > { self . tbs_cert . subject . unwrap_read () } # [doc = " Returns an iterable container over the certificate's extension, or"] # [doc = " an error if the extension set contains a duplicate extension."] pub fn extensions (& self) -> Result < Extensions < 'a > , DuplicateExtensionsError > { self . tbs_cert . extensions () } }
};
}
