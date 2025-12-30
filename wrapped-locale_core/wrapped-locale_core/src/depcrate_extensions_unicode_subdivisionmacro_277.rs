// Generated macro for macro_277 (macro)
macro_rules! Depcrate_extensions_unicode_subdivisionmacro_277 {
() => {
// Module: crate::extensions::unicode::subdivision
// Provides: {"macro_277"}
// Dependencies: {}
impl_tinystr_subtag ! (# [doc = " A subdivision suffix used in [`SubdivisionId`]."] # [doc = ""] # [doc = " This suffix represents a specific subdivision code under a given [`Region`]."] # [doc = " For example the value of [`SubdivisionId`] may be `gbsct`, where the [`SubdivisionSuffix`]"] # [doc = " is `sct` for Scotland."] # [doc = ""] # [doc = " Such a value associated with a key `rg` means that the locale should use Unit Preferences"] # [doc = " (default calendar, currency, week data, time cycle, measurement system) for Scotland, even if the"] # [doc = " [`LanguageIdentifier`](crate::LanguageIdentifier) is `en-US`."] # [doc = ""] # [doc = " A subdivision suffix has to be a sequence of alphanumerical characters no"] # [doc = " shorter than one and no longer than four characters."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::unicode::{subdivision_suffix, SubdivisionSuffix};"] # [doc = ""] # [doc = " let ss: SubdivisionSuffix ="] # [doc = "     \"sct\".parse().expect(\"Failed to parse a SubdivisionSuffix.\");"] # [doc = ""] # [doc = " assert_eq!(ss, subdivision_suffix!(\"sct\"));"] # [doc = " ```"] SubdivisionSuffix , extensions :: unicode , subdivision_suffix , extensions_unicode_subdivision_suffix , 1 ..= 4 , s , s . is_ascii_alphanumeric () , s . to_ascii_lowercase () , s . is_ascii_alphanumeric () && s . is_ascii_lowercase () , InvalidExtension , ["sct"] , ["toolooong"] ,) ;
};
}
