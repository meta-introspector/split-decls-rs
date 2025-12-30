// Generated macro for impl_147 (impl)
macro_rules! Depcrateimpl_147 {
() => {
// Module: crate
// Provides: {"impl_147"}
// Dependencies: {}
impl PrivateKeyDer < '_ > { # [doc = " Clone the private key to a `'static` value"] # [cfg (feature = "alloc")] pub fn clone_key (& self) -> PrivateKeyDer < 'static > { use PrivateKeyDer :: * ; match self { Pkcs1 (key) => Pkcs1 (key . clone_key ()) , Sec1 (key) => Sec1 (key . clone_key ()) , Pkcs8 (key) => Pkcs8 (key . clone_key ()) , } } # [doc = " Yield the DER-encoded bytes of the private key"] pub fn secret_der (& self) -> & [u8] { match self { PrivateKeyDer :: Pkcs1 (key) => key . secret_pkcs1_der () , PrivateKeyDer :: Sec1 (key) => key . secret_sec1_der () , PrivateKeyDer :: Pkcs8 (key) => key . secret_pkcs8_der () , } } }
};
}
