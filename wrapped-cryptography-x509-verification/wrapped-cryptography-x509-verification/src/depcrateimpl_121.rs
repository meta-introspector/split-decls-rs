// Generated macro for impl_121 (impl)
macro_rules! Depcrateimpl_121 {
() => {
// Module: crate
// Provides: {"impl_121"}
// Dependencies: {}
impl < 'chain , B : CryptoOps > ValidationError < 'chain , B > { pub fn new (kind : ValidationErrorKind < 'chain , B >) -> Self { ValidationError { kind , cert : None } } pub (crate) fn set_cert (mut self , cert : VerificationCertificate < 'chain , B >) -> Self { self . cert = Some (cert) ; self } pub fn certificate (& self) -> Option < & VerificationCertificate < 'chain , B > > { self . cert . as_ref () } }
};
}
