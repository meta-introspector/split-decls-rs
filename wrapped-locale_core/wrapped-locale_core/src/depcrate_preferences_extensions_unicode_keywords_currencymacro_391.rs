// Generated macro for macro_391 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_currencymacro_391 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::currency
// Provides: {"macro_391"}
// Dependencies: {}
struct_keyword ! (# [doc = " A Unicode Currency Identifier defines a type of currency."] # [doc = ""] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeCurrencyIdentifier)."] CurrencyType , "cu" , TinyAsciiStr < 3 >, | input : Value | { if let Some (subtag) = input . into_single_subtag () { let ts = subtag . as_tinystr () ; if ts . len () == 3 && ts . is_ascii_alphabetic () { return Ok (Self (ts . resize ())) ; } } Err (PreferencesParseError :: InvalidKeywordValue) } , | input : CurrencyType | { crate :: extensions :: unicode :: Value :: from_subtag (Some (Subtag :: from_tinystr_unvalidated (input . 0 . resize ()) ,)) }) ;
};
}
