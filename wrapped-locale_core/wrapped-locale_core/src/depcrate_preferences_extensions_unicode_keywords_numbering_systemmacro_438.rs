// Generated macro for macro_438 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_numbering_systemmacro_438 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::numbering_system
// Provides: {"macro_438"}
// Dependencies: {}
struct_keyword ! (# [doc = " A Unicode Number System Identifier defines a type of number system."] # [doc = ""] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeNumberSystemIdentifier)."] [Copy] NumberingSystem , "nu" , Subtag , | input : Value | { input . into_single_subtag () . map (Self) . ok_or (PreferencesParseError :: InvalidKeywordValue) } , | input : NumberingSystem | { crate :: extensions :: unicode :: Value :: from_subtag (Some (input . 0)) }) ;
};
}
