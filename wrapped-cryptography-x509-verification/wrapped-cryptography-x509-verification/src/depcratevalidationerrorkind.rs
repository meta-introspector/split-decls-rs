// Generated macro for ValidationErrorKind (enum)
macro_rules! DepcrateValidationErrorKind {
() => {
// Module: crate
// Provides: {"ValidationErrorKind"}
// Dependencies: {}
pub enum ValidationErrorKind < 'chain , B : CryptoOps > { CandidatesExhausted (Box < ValidationError < 'chain , B > >) , Malformed (asn1 :: ParseError) , ExtensionError { oid : ObjectIdentifier , reason : & 'static str , } , FatalError (& 'static str) , Other (String) , }
};
}
