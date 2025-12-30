// Generated macro for permits_subject_alternative_name (function)
macro_rules! Depcrate_policypermits_subject_alternative_name {
() => {
// Module: crate::policy
// Provides: {"permits_subject_alternative_name"}
// Dependencies: {}
fn permits_subject_alternative_name < 'chain , B : CryptoOps > (subject : & Subject < '_ > , san : & Option < SubjectAlternativeName < '_ > > ,) -> ValidationResult < 'chain , () , B > { let Some (san) = san else { return Err (ValidationError :: new (ValidationErrorKind :: Other ("missing required extension: leaf server certificate has no subjectAltName" . into () ,))) ; } ; if ! subject . matches (san) { return Err (ValidationError :: new (ValidationErrorKind :: Other ("leaf certificate has no matching subjectAltName" . into () ,))) ; } Ok (()) }
};
}
