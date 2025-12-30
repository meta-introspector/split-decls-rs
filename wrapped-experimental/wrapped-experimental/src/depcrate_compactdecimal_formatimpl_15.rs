// Generated macro for impl_15 (impl)
macro_rules! Depcrate_compactdecimal_formatimpl_15 {
() => {
// Module: crate::compactdecimal::format
// Provides: {"impl_15"}
// Dependencies: {}
impl FormattedCompactDecimal < '_ > { # [doc = " Access the resolved [`CompactDecimal`] after formatting."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use fixed_decimal::Decimal;"] # [doc = " use icu::experimental::compactdecimal::CompactDecimalFormatter;"] # [doc = " use icu::locale::locale;"] # [doc = " use writeable::assert_writeable_eq;"] # [doc = ""] # [doc = " let short_english = CompactDecimalFormatter::try_new_short("] # [doc = "     locale!(\"en\").into(),"] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .unwrap();"] # [doc = ""] # [doc = " let formatted_compact_decimal = short_english.format_i64(2207);"] # [doc = ""] # [doc = " assert_writeable_eq!(formatted_compact_decimal, \"2.2K\");"] # [doc = " assert_eq!("] # [doc = "     formatted_compact_decimal.get_compact_decimal().to_string(),"] # [doc = "     \"2.2c3\""] # [doc = " );"] # [doc = " ```"] pub fn get_compact_decimal (& self) -> & CompactDecimal { & self . value } }
};
}
