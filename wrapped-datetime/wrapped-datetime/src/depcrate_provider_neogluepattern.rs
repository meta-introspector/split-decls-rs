// Generated macro for GluePattern (struct)
macro_rules! Depcrate_provider_neoGluePattern {
() => {
// Module: crate::provider::neo
// Provides: {"GluePattern"}
// Dependencies: {}
# [doc = " The default per-length patterns used for combining dates, times, and timezones into formatted strings."] # [doc = glue_pattern_v1_size ! ()] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: neo))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [yoke (prove_covariance_manually)] pub struct GluePattern < 'data > { # [doc = " The pattern"] # [cfg_attr (feature = "serde" , serde (borrow))] pub pattern : runtime :: GenericPattern < 'data > , }
};
}
