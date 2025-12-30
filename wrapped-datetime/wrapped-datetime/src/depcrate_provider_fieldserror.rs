// Generated macro for Error (enum)
macro_rules! Depcrate_provider_fieldsError {
() => {
// Module: crate::provider::fields
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error relating to the field for a date pattern field as a whole."] # [doc = ""] # [doc = " Separate error types exist for parts of a field, like the"] # [doc = " [`LengthError`](error for the field length) and the"] # [doc = " [`SymbolError`](error for the field symbol)."] # [derive (Display , Debug , Copy , Clone , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " An error originating inside of the [data provider](icu_provider)."] # [displaydoc ("Field {0:?} is not a valid length")] InvalidLength (FieldSymbol) , }
};
}
