// Generated macro for SymbolError (enum)
macro_rules! Depcrate_provider_fields_symbolsSymbolError {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"SymbolError"}
// Dependencies: {}
# [doc = " An error relating to the field symbol for a date pattern field."] # [derive (Display , Debug , PartialEq , Copy , Clone)] # [non_exhaustive] pub enum SymbolError { # [doc = " Invalid field symbol index."] # [displaydoc ("Invalid field symbol index: {0}")] InvalidIndex (u8) , # [doc = " Unknown field symbol."] # [displaydoc ("Unknown field symbol: {0}")] Unknown (char) , # [doc = " Invalid character for a field symbol."] # [displaydoc ("Invalid character for a field symbol: {0}")] Invalid (u8) , }
};
}
