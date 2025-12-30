// Generated macro for impl_9 (impl)
macro_rules! Depcrate_opsimpl_9 {
() => {
// Module: crate::ops
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , B : CryptoOps > VerificationCertificate < 'a , B > { pub fn new (cert : & 'a Certificate < 'a > , extra : B :: CertificateExtra) -> Self { VerificationCertificate { cert , extra , public_key : OnceLock :: new () , } } pub fn certificate (& self) -> & Certificate < 'a > { self . cert } pub fn public_key (& self , ops : & B) -> Result < & B :: Key , B :: Err > { if let Some (key) = self . public_key . get () { return Ok (key) ; } let key = ops . public_key (self . certificate ()) ? ; Ok (self . public_key . get_or_init (| | key)) } pub fn extra (& self) -> & B :: CertificateExtra { & self . extra } }
};
}
