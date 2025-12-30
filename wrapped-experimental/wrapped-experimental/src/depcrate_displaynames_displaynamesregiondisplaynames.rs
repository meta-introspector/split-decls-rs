// Generated macro for RegionDisplayNames (struct)
macro_rules! Depcrate_displaynames_displaynamesRegionDisplayNames {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"RegionDisplayNames"}
// Dependencies: {}
# [doc = " Lookup of the locale-specific display names by region code."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::experimental::displaynames::{"] # [doc = "     DisplayNamesOptions, RegionDisplayNames,"] # [doc = " };"] # [doc = " use icu::locale::{locale, subtags::region};"] # [doc = ""] # [doc = " let locale = locale!(\"en-001\").into();"] # [doc = " let options: DisplayNamesOptions = Default::default();"] # [doc = " let display_name = RegionDisplayNames::try_new(locale, options)"] # [doc = "     .expect(\"Data should load successfully\");"] # [doc = ""] # [doc = " assert_eq!(display_name.of(region!(\"AE\")), Some(\"United Arab Emirates\"));"] # [doc = " ```"] # [derive (Default)] pub struct RegionDisplayNames { options : DisplayNamesOptions , region_data : DataPayload < RegionDisplayNamesV1 > , }
};
}
