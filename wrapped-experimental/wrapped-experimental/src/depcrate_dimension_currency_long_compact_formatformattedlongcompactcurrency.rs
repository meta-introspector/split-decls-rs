// Generated macro for FormattedLongCompactCurrency (struct)
macro_rules! Depcrate_dimension_currency_long_compact_formatFormattedLongCompactCurrency {
() => {
// Module: crate::dimension::currency::long_compact_format
// Provides: {"FormattedLongCompactCurrency"}
// Dependencies: {}
pub struct FormattedLongCompactCurrency < 'l > { pub (crate) signed_fixed_decimal : & 'l Decimal , pub (crate) _currency_code : CurrencyCode , pub (crate) extended : & 'l CurrencyExtendedData < 'l > , pub (crate) patterns : & 'l CurrencyPatternsData < 'l > , pub (crate) compact_decimal_formatter : & 'l CompactDecimalFormatter , pub (crate) plural_rules : & 'l PluralRules , }
};
}
