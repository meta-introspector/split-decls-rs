// Generated macro for ValidationError (struct)
macro_rules! DepcrateValidationError {
() => {
// Module: crate
// Provides: {"ValidationError"}
// Dependencies: {}
pub struct ValidationError < 'chain , B : CryptoOps > { kind : ValidationErrorKind < 'chain , B > , cert : Option < VerificationCertificate < 'chain , B > > , }
};
}
