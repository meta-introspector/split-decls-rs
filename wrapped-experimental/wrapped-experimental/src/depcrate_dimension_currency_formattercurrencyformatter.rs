// Generated macro for CurrencyFormatter (struct)
macro_rules! Depcrate_dimension_currency_formatterCurrencyFormatter {
() => {
// Module: crate::dimension::currency::formatter
// Provides: {"CurrencyFormatter"}
// Dependencies: {}
# [doc = " A formatter for monetary values."] # [doc = ""] # [doc = " [`CurrencyFormatter`] supports:"] # [doc = "   1. Rendering in the locale's currency system."] # [doc = "   2. Locale-sensitive grouping separator positions."] # [doc = ""] # [doc = " Read more about the options in the [`super::options`] module."] pub struct CurrencyFormatter { # [doc = " Options bag for the currency formatter to determine the behavior of the formatter."] # [doc = " for example: currency width."] options : CurrencyFormatterOptions , # [doc = " Essential data for the currency formatter."] essential : DataPayload < CurrencyEssentialsV1 > , # [doc = " A [`DecimalFormatter`] to format the currency value."] decimal_formatter : DecimalFormatter , }
};
}
