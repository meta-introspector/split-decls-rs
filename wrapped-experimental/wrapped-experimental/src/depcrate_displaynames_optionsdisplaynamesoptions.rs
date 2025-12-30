// Generated macro for DisplayNamesOptions (struct)
macro_rules! Depcrate_displaynames_optionsDisplayNamesOptions {
() => {
// Module: crate::displaynames::options
// Provides: {"DisplayNamesOptions"}
// Dependencies: {}
# [doc = " A bag of options defining how region or language codes will be translated."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::experimental::displaynames::{"] # [doc = "     DisplayNamesOptions, RegionDisplayNames, Style,"] # [doc = " };"] # [doc = " use icu::locale::{locale, subtags::region};"] # [doc = ""] # [doc = " let locale = locale!(\"en-001\");"] # [doc = " let mut options: DisplayNamesOptions = Default::default();"] # [doc = " options.style = Some(Style::Short);"] # [doc = " let display_name = RegionDisplayNames::try_new(locale.into(), options)"] # [doc = "     .expect(\"Data should load successfully\");"] # [doc = ""] # [doc = " // Full name would be \"Bosnia & Herzegovina\""] # [doc = " assert_eq!(display_name.of(region!(\"BA\")), Some(\"Bosnia\"));"] # [doc = " ```"] # [derive (Copy , Debug , Eq , PartialEq , Clone , Default)] # [non_exhaustive] pub struct DisplayNamesOptions { # [doc = " The optional formatting style to use for display name."] pub style : Option < Style > , # [doc = " The fallback return when the system does not have the"] # [doc = " requested display name, defaults to \"code\"."] pub fallback : Fallback , # [doc = " The language display kind, defaults to \"dialect\"."] pub language_display : LanguageDisplay , }
};
}
