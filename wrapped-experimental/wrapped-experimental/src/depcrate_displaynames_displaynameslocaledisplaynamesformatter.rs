// Generated macro for LocaleDisplayNamesFormatter (struct)
macro_rules! Depcrate_displaynames_displaynamesLocaleDisplayNamesFormatter {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"LocaleDisplayNamesFormatter"}
// Dependencies: {}
# [doc = " Format a locale as a display string."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::experimental::displaynames::{"] # [doc = "     DisplayNamesOptions, LocaleDisplayNamesFormatter,"] # [doc = " };"] # [doc = " use icu::locale::locale;"] # [doc = ""] # [doc = " let locale = locale!(\"en-001\").into();"] # [doc = " let options: DisplayNamesOptions = Default::default();"] # [doc = " let display_name = LocaleDisplayNamesFormatter::try_new(locale, options)"] # [doc = "     .expect(\"Data should load successfully\");"] # [doc = ""] # [doc = " assert_eq!(display_name.of(&locale!(\"en-GB\")), \"British English\");"] # [doc = " assert_eq!(display_name.of(&locale!(\"en\")), \"English\");"] # [doc = " assert_eq!(display_name.of(&locale!(\"en-MX\")), \"English (Mexico)\");"] # [doc = " assert_eq!(display_name.of(&locale!(\"xx-YY\")), \"xx (YY)\");"] # [doc = " assert_eq!(display_name.of(&locale!(\"xx\")), \"xx\");"] # [doc = " ```"] pub struct LocaleDisplayNamesFormatter { options : DisplayNamesOptions , locale_data : DataPayload < LocaleDisplayNamesV1 > , language_data : DataPayload < LanguageDisplayNamesV1 > , script_data : DataPayload < ScriptDisplayNamesV1 > , region_data : DataPayload < RegionDisplayNamesV1 > , variant_data : DataPayload < VariantDisplayNamesV1 > , }
};
}
