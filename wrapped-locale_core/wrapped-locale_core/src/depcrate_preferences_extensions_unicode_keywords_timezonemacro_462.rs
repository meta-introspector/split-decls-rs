// Generated macro for macro_462 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_timezonemacro_462 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::timezone
// Provides: {"macro_462"}
// Dependencies: {}
struct_keyword ! (# [doc = " A Unicode Timezone Identifier defines a timezone."] # [doc = ""] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeTimezoneIdentifier)."] [Copy] TimeZoneShortId , "tz" , Subtag , | input : Value | { input . into_single_subtag () . map (Self) . ok_or (PreferencesParseError :: InvalidKeywordValue) } , | input : TimeZoneShortId | { crate :: extensions :: unicode :: Value :: from_subtag (Some (input . 0)) }) ;
};
}
