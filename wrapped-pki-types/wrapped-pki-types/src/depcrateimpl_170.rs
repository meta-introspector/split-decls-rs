// Generated macro for impl_170 (impl)
macro_rules! Depcrateimpl_170 {
() => {
// Module: crate
// Provides: {"impl_170"}
// Dependencies: {}
impl PrivatePkcs8KeyDer < '_ > { # [doc = " Clone the private key to a `'static` value"] # [cfg (feature = "alloc")] pub fn clone_key (& self) -> PrivatePkcs8KeyDer < 'static > { PrivatePkcs8KeyDer :: from (self . 0 . as_ref () . to_vec ()) } # [doc = " Yield the DER-encoded bytes of the private key"] pub fn secret_pkcs8_der (& self) -> & [u8] { self . 0 . as_ref () } }
};
}
