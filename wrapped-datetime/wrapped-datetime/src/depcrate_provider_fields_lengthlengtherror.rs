// Generated macro for LengthError (enum)
macro_rules! Depcrate_provider_fields_lengthLengthError {
() => {
// Module: crate::provider::fields::length
// Provides: {"LengthError"}
// Dependencies: {}
# [doc = " An error relating to the length of a field within a date pattern."] # [derive (Display , Debug , PartialEq , Copy , Clone)] # [non_exhaustive] pub enum LengthError { # [doc = " The length of the field string within the pattern is invalid, according to"] # [doc = " the field type and its supported field patterns in LDML. See [`FieldLength`]."] # [displaydoc ("Invalid length")] InvalidLength , }
};
}
