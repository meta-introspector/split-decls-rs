// Generated macro for LongCurrencyFormatter (struct)
macro_rules! Depcrate_dimension_currency_long_formatterLongCurrencyFormatter {
() => {
// Module: crate::dimension::currency::long_formatter
// Provides: {"LongCurrencyFormatter"}
// Dependencies: {}
# [doc = " A formatter for monetary values."] # [doc = ""] # [doc = " [`LongCurrencyFormatter`] supports:"] # [doc = "   1. Rendering in the locale's currency system."] # [doc = "   2. Locale-sensitive grouping separator positions."] # [doc = ""] # [doc = " Read more about the options in the [`super::options`] module."] pub struct LongCurrencyFormatter { # [doc = " Extended data for the currency formatter."] extended : DataPayload < CurrencyExtendedDataV1 > , # [doc = " Formatting patterns for each currency plural category."] patterns : DataPayload < CurrencyPatternsDataV1 > , # [doc = " A [`DecimalFormatter`] to format the currency value."] decimal_formatter : DecimalFormatter , # [doc = " A [`PluralRules`] to determine the plural category of the unit."] plural_rules : PluralRules , }
};
}
