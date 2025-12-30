// Generated macro for VariantDisplayNames (struct)
macro_rules! Depcrate_displaynames_displaynamesVariantDisplayNames {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"VariantDisplayNames"}
// Dependencies: {}
# [doc = " Lookup of the locale-specific display names by variant."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::experimental::displaynames::{"] # [doc = "     DisplayNamesOptions, VariantDisplayNames,"] # [doc = " };"] # [doc = " use icu::locale::{locale, subtags::variant};"] # [doc = ""] # [doc = " let locale = locale!(\"en-001\").into();"] # [doc = " let options: DisplayNamesOptions = Default::default();"] # [doc = " let display_name = VariantDisplayNames::try_new(locale, options)"] # [doc = "     .expect(\"Data should load successfully\");"] # [doc = ""] # [doc = " assert_eq!(display_name.of(variant!(\"POSIX\")), Some(\"Computer\"));"] # [doc = " ```"] # [derive (Default)] pub struct VariantDisplayNames { # [allow (dead_code)] options : DisplayNamesOptions , variant_data : DataPayload < VariantDisplayNamesV1 > , }
};
}
