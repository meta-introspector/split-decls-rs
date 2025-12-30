// Generated macro for macro_428 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_measurement_systemmacro_428 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::measurement_system
// Provides: {"macro_428"}
// Dependencies: {}
enum_keyword ! (# [doc = " A Unicode Measurement System Identifier defines a preferred measurement system."] # [doc = ""] # [doc = " Specifying \"ms\" in a locale identifier overrides the default value specified by supplemental measurement system data for the region"] # [doc = ""] # [doc = " The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeMeasurementSystemIdentifier)."] MeasurementSystem { # [doc = " Metric System"] ("metric" => Metric) , # [doc = " US System of measurement: feet, pints, etc.; pints are 16oz"] ("ussystem" => USSystem) , # [doc = " UK System of measurement: feet, pints, etc.; pints are 20oz"] ("uksystem" => UKSystem) } , "ms") ;
};
}
