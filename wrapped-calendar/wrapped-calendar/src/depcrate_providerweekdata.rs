// Generated macro for WeekData (struct)
macro_rules! Depcrate_providerWeekData {
() => {
// Module: crate::provider
// Provides: {"WeekData"}
// Dependencies: {}
# [doc = " An ICU4X mapping to a subset of CLDR weekData."] # [doc = " See CLDR-JSON's weekData.json for more context."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , Copy , Debug , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_calendar :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] pub struct WeekData { # [doc = " The first day of a week."] pub first_weekday : Weekday , # [doc = " Bitset representing weekdays that are part of the 'weekend', for calendar purposes."] # [doc = " The number of days can be different between locales, and may not be contiguous."] pub weekend : WeekdaySet , }
};
}
