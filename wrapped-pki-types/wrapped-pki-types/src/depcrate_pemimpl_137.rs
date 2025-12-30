// Generated macro for impl_137 (impl)
macro_rules! Depcrate_pemimpl_137 {
() => {
// Module: crate::pem
// Provides: {"impl_137"}
// Dependencies: {}
impl SectionKind { const fn secret (& self) -> bool { match self { Self :: RsaPrivateKey | Self :: PrivateKey | Self :: EcPrivateKey => true , Self :: Certificate | Self :: PublicKey | Self :: Crl | Self :: Csr | Self :: EchConfigList => { false } } } fn as_slice (& self) -> & 'static [u8] { match self { Self :: Certificate => b"CERTIFICATE" , Self :: PublicKey => b"PUBLIC KEY" , Self :: RsaPrivateKey => b"RSA PRIVATE KEY" , Self :: PrivateKey => b"PRIVATE KEY" , Self :: EcPrivateKey => b"EC PRIVATE KEY" , Self :: Crl => b"X509 CRL" , Self :: Csr => b"CERTIFICATE REQUEST" , Self :: EchConfigList => b"ECHCONFIG" , } } }
};
}
