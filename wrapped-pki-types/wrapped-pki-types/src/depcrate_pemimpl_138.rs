// Generated macro for impl_138 (impl)
macro_rules! Depcrate_pemimpl_138 {
() => {
// Module: crate::pem
// Provides: {"impl_138"}
// Dependencies: {}
impl TryFrom < & [u8] > for SectionKind { type Error = () ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { Ok (match value { b"CERTIFICATE" => Self :: Certificate , b"PUBLIC KEY" => Self :: PublicKey , b"RSA PRIVATE KEY" => Self :: RsaPrivateKey , b"PRIVATE KEY" => Self :: PrivateKey , b"EC PRIVATE KEY" => Self :: EcPrivateKey , b"X509 CRL" => Self :: Crl , b"CERTIFICATE REQUEST" => Self :: Csr , b"ECHCONFIG" => Self :: EchConfigList , _ => return Err (()) , }) } }
};
}
