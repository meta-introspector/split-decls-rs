// Generated macro for VerificationCertificate (struct)
macro_rules! Depcrate_opsVerificationCertificate {
() => {
// Module: crate::ops
// Provides: {"VerificationCertificate"}
// Dependencies: {}
pub struct VerificationCertificate < 'a , B : CryptoOps > { cert : & 'a Certificate < 'a > , public_key : OnceLock < B :: Key > , extra : B :: CertificateExtra , }
};
}
