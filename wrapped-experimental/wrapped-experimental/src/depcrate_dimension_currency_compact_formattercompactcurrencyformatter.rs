// Generated macro for CompactCurrencyFormatter (struct)
macro_rules! Depcrate_dimension_currency_compact_formatterCompactCurrencyFormatter {
() => {
// Module: crate::dimension::currency::compact_formatter
// Provides: {"CompactCurrencyFormatter"}
// Dependencies: {}
# [doc = " A formatter for monetary values."] # [doc = ""] # [doc = " [`CompactCurrencyFormatter`] supports:"] # [doc = "   1. Rendering in the locale's currency system."] # [doc = "   2. Locale-sensitive grouping separator positions."] # [doc = ""] # [doc = " Read more about the options in the [`super::compact_options`] module."] pub struct CompactCurrencyFormatter { # [doc = " Short currency compact data for the compact currency formatter."] short_currency_compact : DataPayload < ShortCurrencyCompactV1 > , # [doc = " Essential data for the compact currency formatter."] essential : DataPayload < CurrencyEssentialsV1 > , # [doc = " A [`CompactDecimalFormatter`] to format the currency value."] compact_decimal_formatter : CompactDecimalFormatter , # [doc = " Options bag for the compact currency formatter to determine the behavior of the formatter."] # [doc = " for example: width."] options : CompactCurrencyFormatterOptions , }
};
}
