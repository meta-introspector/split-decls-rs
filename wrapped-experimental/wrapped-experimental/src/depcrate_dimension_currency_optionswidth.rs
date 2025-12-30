// Generated macro for Width (enum)
macro_rules! Depcrate_dimension_currency_optionsWidth {
() => {
// Module: crate::dimension::currency::options
// Provides: {"Width"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq , Clone , Copy , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [non_exhaustive] pub enum Width { # [doc = " Format the currency with the standard (short) currency symbol."] # [doc = ""] # [doc = " For example, 1 USD formats as \"$1.00\" in en-US and \"US$1\" in most other locales."] # [cfg_attr (feature = "serde" , serde (rename = "short"))] Short , # [doc = " Format the currency with the narrow currency symbol."] # [doc = ""] # [doc = " The narrow symbol may be ambiguous, so it should be evident from context which"] # [doc = " currency is being represented."] # [doc = ""] # [doc = " For example, 1 USD formats as \"$1.00\" in most locales."] # [cfg_attr (feature = "serde" , serde (rename = "narrow"))] Narrow , }
};
}
