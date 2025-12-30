// Generated macro for impl_13 (impl)
macro_rules! Depcrate_opsimpl_13 {
() => {
// Module: crate::ops
// Provides: {"impl_13"}
// Dependencies: {}
impl < B : CryptoOps > Clone for VerificationCertificate < '_ , B > { fn clone (& self) -> Self { Self { cert : self . cert , extra : B :: clone_extra (& self . extra) , public_key : { let cell = OnceLock :: new () ; if let Some (k) = self . public_key . get () { cell . set (B :: clone_public_key (k)) . ok () . unwrap () ; } cell } , } } }
};
}
