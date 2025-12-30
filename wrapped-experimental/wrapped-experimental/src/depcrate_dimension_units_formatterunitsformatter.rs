// Generated macro for UnitsFormatter (struct)
macro_rules! Depcrate_dimension_units_formatterUnitsFormatter {
() => {
// Module: crate::dimension::units::formatter
// Provides: {"UnitsFormatter"}
// Dependencies: {}
# [doc = " A formatter for measurement unit values."] # [doc = ""] # [doc = " [`UnitsFormatter`] supports:"] # [doc = "   1. Rendering in the locale's units system."] # [doc = "   2. Locale-sensitive grouping separator positions."] # [doc = ""] # [doc = " Read more about the options in the [`super::options`] module."] pub struct UnitsFormatter { # [doc = " Options bag for the units formatter to determine the behavior of the formatter."] # [doc = " for example: width of the units."] _options : UnitsFormatterOptions , # [doc = " Display name for the units."] display_name : DataPayload < UnitsDisplayNamesV1 > , # [doc = " A [`DecimalFormatter`] to format the unit value."] decimal_formatter : DecimalFormatter , # [doc = " A [`PluralRules`] to determine the plural category of the unit."] plural_rules : PluralRules , }
};
}
