// Generated macro for LanguageDisplayNames (struct)
macro_rules! Depcrate_displaynames_displaynamesLanguageDisplayNames {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"LanguageDisplayNames"}
// Dependencies: {}
# [doc = " Lookup of the locale-specific display names by language code."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::experimental::displaynames::{"] # [doc = "     DisplayNamesOptions, LanguageDisplayNames,"] # [doc = " };"] # [doc = " use icu::locale::{locale, subtags::language};"] # [doc = ""] # [doc = " let locale = locale!(\"en-001\").into();"] # [doc = " let options: DisplayNamesOptions = Default::default();"] # [doc = " let display_name = LanguageDisplayNames::try_new(locale, options)"] # [doc = "     .expect(\"Data should load successfully\");"] # [doc = ""] # [doc = " assert_eq!(display_name.of(language!(\"de\")), Some(\"German\"));"] # [doc = " ```"] # [derive (Default)] pub struct LanguageDisplayNames { options : DisplayNamesOptions , language_data : DataPayload < LanguageDisplayNamesV1 > , }
};
}
