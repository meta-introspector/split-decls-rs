// Generated macro for LongCompactCurrencyFormatter (struct)
macro_rules! Depcrate_dimension_currency_long_compact_formatterLongCompactCurrencyFormatter {
() => {
// Module: crate::dimension::currency::long_compact_formatter
// Provides: {"LongCompactCurrencyFormatter"}
// Dependencies: {}
# [doc = " A formatter for monetary values."] # [doc = ""] # [doc = " [`LongCompactCurrencyFormatter`] supports:"] # [doc = "   1. Rendering in the locale's currency system."] # [doc = "   2. Locale-sensitive grouping separator positions."] pub struct LongCompactCurrencyFormatter { # [doc = " Extended data for the currency formatter."] extended : DataPayload < CurrencyExtendedDataV1 > , # [doc = " Formatting patterns for each currency plural category."] patterns : DataPayload < CurrencyPatternsDataV1 > , # [doc = " A [`CompactDecimalFormatter`] to format the currency value in compact form."] compact_decimal_formatter : CompactDecimalFormatter , # [doc = " A [`PluralRules`] to determine the plural category of the unit."] plural_rules : PluralRules , }
};
}
