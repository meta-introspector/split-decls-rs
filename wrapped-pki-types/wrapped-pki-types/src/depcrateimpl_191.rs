// Generated macro for impl_191 (impl)
macro_rules! Depcrateimpl_191 {
() => {
// Module: crate
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'a > CertificateDer < 'a > { # [doc = " A const constructor to create a `CertificateDer` from a slice of DER."] pub const fn from_slice (bytes : & 'a [u8]) -> Self { Self (Der :: from_slice (bytes)) } }
};
}
