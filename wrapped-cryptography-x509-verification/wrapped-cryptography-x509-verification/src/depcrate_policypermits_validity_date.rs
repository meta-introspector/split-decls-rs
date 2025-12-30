// Generated macro for permits_validity_date (function)
macro_rules! Depcrate_policypermits_validity_date {
() => {
// Module: crate::policy
// Provides: {"permits_validity_date"}
// Dependencies: {}
fn permits_validity_date < 'chain , B : CryptoOps > (validity_date : & Time ,) -> ValidationResult < 'chain , () , B > { const GENERALIZED_DATE_INVALIDITY_RANGE : Range < u16 > = 1950 .. 2050 ; if let Time :: GeneralizedTime (_) = validity_date { if GENERALIZED_DATE_INVALIDITY_RANGE . contains (& validity_date . as_datetime () . year ()) { return Err (ValidationError :: new (ValidationErrorKind :: Other ("validity dates between 1950 and 2049 must be UtcTime" . to_string () ,))) ; } } Ok (()) }
};
}
