// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use asn1 :: ParseError ; use cryptography_x509 :: oid :: SUBJECT_ALTERNATIVE_NAME_OID ; use crate :: certificate :: tests :: PublicKeyErrorOps ; use crate :: { ValidationError , ValidationErrorKind } ; # [test] fn test_validationerror_display () { let err = ValidationError :: < PublicKeyErrorOps > :: new (ValidationErrorKind :: Malformed (ParseError :: new (asn1 :: ParseErrorKind :: InvalidLength) ,)) ; assert_eq ! (err . to_string () , "ASN.1 parsing error: invalid length") ; let err = ValidationError :: < PublicKeyErrorOps > :: new (ValidationErrorKind :: ExtensionError { oid : SUBJECT_ALTERNATIVE_NAME_OID , reason : "duplicate extension" , }) ; assert_eq ! (err . to_string () , "invalid extension: 2.5.29.17: duplicate extension") ; let err = ValidationError :: < PublicKeyErrorOps > :: new (ValidationErrorKind :: FatalError ("oops")) ; assert_eq ! (err . to_string () , "fatal error: oops") ; } }
};
}
