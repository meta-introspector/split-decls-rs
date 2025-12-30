// Generated macro for macro_444 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_region_overridemacro_444 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::region_override
// Provides: {"macro_444"}
// Dependencies: {}
struct_keyword ! (# [doc = " A Region Override specifies an alternate region to use for obtaining certain region-specific default values."] # [doc = ""] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#RegionOverride)."] [Copy] RegionOverride , "rg" , SubdivisionId , | input : Value | { input . into_single_subtag () . and_then (| subtag | subtag . as_str () . parse () . ok () . map (Self)) . ok_or (PreferencesParseError :: InvalidKeywordValue) } , | input : RegionOverride | { Value :: from_subtag (Some (input . 0 . into_subtag ())) }) ;
};
}
