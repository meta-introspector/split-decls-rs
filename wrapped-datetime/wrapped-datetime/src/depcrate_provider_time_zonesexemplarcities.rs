// Generated macro for ExemplarCities (struct)
macro_rules! Depcrate_provider_time_zonesExemplarCities {
() => {
// Module: crate::provider::time_zones
// Provides: {"ExemplarCities"}
// Dependencies: {}
# [doc = " An ICU4X mapping to the CLDR timeZoneNames exemplar cities."] # [doc = " See CLDR-JSON timeZoneNames.json for more context."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (PartialEq , Debug , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: time_zones))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] pub struct ExemplarCities < 'data > { # [doc = " Per-zone exemplar city name. This is deduplicated against `Locations.locations`, so it"] # [doc = " only contains time zones that don't use the exemplar city in the location format."] # [cfg_attr (feature = "serde" , serde (borrow))] pub exemplars : ZeroMap < 'data , TimeZone , str > , }
};
}
