// Generated macro for ClaimsValidationRules (struct)
macro_rules! Depcrate_claimsClaimsValidationRules {
() => {
// Module: crate::claims
// Provides: {"ClaimsValidationRules"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone)] # [doc = " The validation rules that are used to validate a set of [`Claims`]."] pub struct ClaimsValidationRules { validate_currently_valid : bool , allow_non_expiring : bool , validate_issuer : Option < String > , validate_subject : Option < String > , validate_audience : Option < String > , validate_token_identifier : Option < String > , }
};
}
