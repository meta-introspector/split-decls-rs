// Generated macro for impl_210 (impl)
macro_rules! Depcrate_certimpl_210 {
() => {
// Module: crate::cert
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'a > CertX509 < 'a > { # [doc = " Return the X.509 certificate data as a byte slice"] pub fn data (& self) -> & [u8] { unsafe { slice :: from_raw_parts ((* self . raw) . data as * const u8 , (* self . raw) . len as usize) } } }
};
}
