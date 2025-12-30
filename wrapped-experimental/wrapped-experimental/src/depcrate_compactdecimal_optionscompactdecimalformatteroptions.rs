// Generated macro for CompactDecimalFormatterOptions (struct)
macro_rules! Depcrate_compactdecimal_optionsCompactDecimalFormatterOptions {
() => {
// Module: crate::compactdecimal::options
// Provides: {"CompactDecimalFormatterOptions"}
// Dependencies: {}
# [doc = " A bag of options defining how numbers will be formatted by"] # [doc = " [`CompactDecimalFormatter`](super::CompactDecimalFormatter)."] # [derive (Debug , Eq , PartialEq , Clone)] # [non_exhaustive] pub struct CompactDecimalFormatterOptions { # [doc = " Options to configure the inner [`icu_decimal::DecimalFormatter`]."] pub decimal_formatter_options : DecimalFormatterOptions , }
};
}
