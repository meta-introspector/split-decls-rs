// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl Certificate { # [doc = " Parses a DER-formatted X509 certificate."] pub fn from_der (der : & [u8]) -> Result < Certificate > { let cert = imp :: Certificate :: from_der (der) ? ; Ok (Certificate (cert)) } # [doc = " Parses a PEM-formatted X509 certificate."] pub fn from_pem (pem : & [u8]) -> Result < Certificate > { let cert = imp :: Certificate :: from_pem (pem) ? ; Ok (Certificate (cert)) } # [doc = " Returns the DER-encoded representation of this certificate."] pub fn to_der (& self) -> Result < Vec < u8 > > { let der = self . 0 . to_der () ? ; Ok (der) } }
};
}
