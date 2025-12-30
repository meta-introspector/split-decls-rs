// Generated macro for impl_156 (impl)
macro_rules! Depcrateimpl_156 {
() => {
// Module: crate
// Provides: {"impl_156"}
// Dependencies: {}
impl PrivatePkcs1KeyDer < '_ > { # [doc = " Clone the private key to a `'static` value"] # [cfg (feature = "alloc")] pub fn clone_key (& self) -> PrivatePkcs1KeyDer < 'static > { PrivatePkcs1KeyDer :: from (self . 0 . as_ref () . to_vec ()) } # [doc = " Yield the DER-encoded bytes of the private key"] pub fn secret_pkcs1_der (& self) -> & [u8] { self . 0 . as_ref () } }
};
}
