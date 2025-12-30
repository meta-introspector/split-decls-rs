// Generated macro for impl_163 (impl)
macro_rules! Depcrateimpl_163 {
() => {
// Module: crate
// Provides: {"impl_163"}
// Dependencies: {}
impl PrivateSec1KeyDer < '_ > { # [doc = " Clone the private key to a `'static` value"] # [cfg (feature = "alloc")] pub fn clone_key (& self) -> PrivateSec1KeyDer < 'static > { PrivateSec1KeyDer :: from (self . 0 . as_ref () . to_vec ()) } # [doc = " Yield the DER-encoded bytes of the private key"] pub fn secret_sec1_der (& self) -> & [u8] { self . 0 . as_ref () } }
};
}
