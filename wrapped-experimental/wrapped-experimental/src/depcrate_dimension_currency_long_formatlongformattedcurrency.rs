// Generated macro for LongFormattedCurrency (struct)
macro_rules! Depcrate_dimension_currency_long_formatLongFormattedCurrency {
() => {
// Module: crate::dimension::currency::long_format
// Provides: {"LongFormattedCurrency"}
// Dependencies: {}
pub struct LongFormattedCurrency < 'l > { pub (crate) value : & 'l Decimal , pub (crate) _currency_code : CurrencyCode , pub (crate) extended : & 'l CurrencyExtendedData < 'l > , pub (crate) patterns : & 'l CurrencyPatternsData < 'l > , pub (crate) decimal_formatter : & 'l DecimalFormatter , pub (crate) plural_rules : & 'l PluralRules , }
};
}
