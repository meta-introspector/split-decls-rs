// Generated macro for DecimalFormatter (struct)
macro_rules! DepcrateDecimalFormatter {
() => {
// Module: crate
// Provides: {"DecimalFormatter"}
// Dependencies: {}
# [doc = " A formatter for [`Decimal`], rendering decimal digits in an i18n-friendly way."] # [doc = ""] # [doc = " [`DecimalFormatter`] supports:"] # [doc = ""] # [doc = " 1. Rendering in the local numbering system"] # [doc = " 2. Locale-sensitive grouping separator positions"] # [doc = " 3. Locale-sensitive plus and minus signs"] # [doc = ""] # [doc = " To get the resolved numbering system, see [`provider`]."] # [doc = ""] # [doc = " See the crate-level documentation for examples."] # [doc = decimal_formatter_size ! ()] # [derive (Debug , Clone)] pub struct DecimalFormatter { options : options :: DecimalFormatterOptions , symbols : DataPayload < provider :: DecimalSymbolsV1 > , digits : DataPayload < provider :: DecimalDigitsV1 > , }
};
}
