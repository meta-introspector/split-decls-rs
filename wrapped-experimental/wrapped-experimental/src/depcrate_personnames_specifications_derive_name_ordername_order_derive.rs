// Generated macro for name_order_derive (function)
macro_rules! Depcrate_personnames_specifications_derive_name_ordername_order_derive {
() => {
// Module: crate::personnames::specifications::derive_name_order
// Provides: {"name_order_derive"}
// Dependencies: {}
# [doc = ""] # [doc = " https://www.unicode.org/reports/tr35/tr35-personNames.html#derive-the-name-order"] pub fn name_order_derive (person_name_locale : & Locale , surname_first : & VarZeroVec < str > , given_first : & VarZeroVec < str > , fallbacker : LocaleFallbackerBorrowed ,) -> FormattingOrder { let mut fallback_iterator = fallbacker . for_config (Default :: default ()) . fallback_for (person_name_locale . into ()) ; loop { let chain_locale = fallback_iterator . get () ; let chain_locale_str = chain_locale . write_to_string () ; let mut chain_locale_und = * chain_locale ; chain_locale_und . language = Language :: UNKNOWN ; let chain_locale_und_str = chain_locale_und . write_to_string () ; if given_first . iter () . any (| i | i == chain_locale_str || i == chain_locale_und_str) { return FormattingOrder :: GivenFirst ; } if surname_first . iter () . any (| i | i == chain_locale_str || i == chain_locale_und_str) { return FormattingOrder :: SurnameFirst ; } fallback_iterator . step () ; } }
};
}
