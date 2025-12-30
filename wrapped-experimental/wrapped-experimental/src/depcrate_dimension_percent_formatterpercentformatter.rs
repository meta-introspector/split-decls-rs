// Generated macro for PercentFormatter (struct)
macro_rules! Depcrate_dimension_percent_formatterPercentFormatter {
() => {
// Module: crate::dimension::percent::formatter
// Provides: {"PercentFormatter"}
// Dependencies: {}
# [doc = " A formatter for percent values."] # [doc = ""] # [doc = " [`PercentFormatter`] supports:"] # [doc = "   1. Rendering in the locale's percent system."] pub struct PercentFormatter < R > { # [doc = " Essential data for the percent formatter."] essential : DataPayload < PercentEssentialsV1 > , # [doc = " Options bag for the percent formatter to determine the behavior of the formatter."] options : PercentFormatterOptions , # [doc = " A fixed decimal formatter used to format the percent value."] decimal_formatter : R , }
};
}
