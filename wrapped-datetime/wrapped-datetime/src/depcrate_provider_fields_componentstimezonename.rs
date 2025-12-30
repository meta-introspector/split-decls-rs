// Generated macro for TimeZoneName (enum)
macro_rules! Depcrate_provider_fields_componentsTimeZoneName {
() => {
// Module: crate::provider::fields::components
// Provides: {"TimeZoneName"}
// Dependencies: {}
# [doc = " Options for displaying a time zone for the `components::`[`Bag`]."] # [doc = ""] # [doc = " Note that the initial implementation is focusing on only supporting ECMA-402 compatible"] # [doc = " options."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature"] # [doc = " of the icu meta-crate. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/1317\">#1317</a>"] # [doc = " </div>"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize) , serde (rename_all = "kebab-case"))] # [non_exhaustive] pub enum TimeZoneName { # [doc = " Short localized form, without the location. (e.g.: PST, GMT-8)"] ShortSpecific , # [doc = " Long localized form, without the location (e.g., Pacific Standard Time, Nordamerikanische Westküsten-Normalzeit)"] LongSpecific , # [doc = " Long localized offset form, e.g. GMT-08:00"] LongOffset , # [doc = " Short localized offset form, e.g. GMT-8"] ShortOffset , # [doc = " Short generic non-location format (e.g.: PT, Los Angeles, Zeit)."] ShortGeneric , # [doc = " Long generic non-location format (e.g.: Pacific Time, Nordamerikanische Westküstenzeit),"] LongGeneric , }
};
}
