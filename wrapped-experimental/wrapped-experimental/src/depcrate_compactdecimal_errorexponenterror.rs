// Generated macro for ExponentError (struct)
macro_rules! Depcrate_compactdecimal_errorExponentError {
() => {
// Module: crate::compactdecimal::error
// Provides: {"ExponentError"}
// Dependencies: {}
# [doc = " An error due to a [`CompactDecimal`](fixed_decimal::CompactDecimal) with an"] # [doc = " exponent inconsistent with the compact decimal data for the locale, e.g.,"] # [doc = " when formatting 1c5 in English (US)."] # [derive (Display , Copy , Clone , Debug)] # [displaydoc ("Expected compact exponent {expected} for 10^{log10_type}, got {actual}")] pub struct ExponentError { # [doc = " The compact decimal exponent passed to the formatter."] pub (crate) actual : u8 , # [doc = " The appropriate compact decimal exponent for a number of the given magnitude."] pub (crate) expected : u8 , # [doc = " The magnitude of the number being formatted."] pub (crate) log10_type : i16 , }
};
}
