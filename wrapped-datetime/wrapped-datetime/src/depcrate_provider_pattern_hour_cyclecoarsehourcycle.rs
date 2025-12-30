// Generated macro for CoarseHourCycle (enum)
macro_rules! Depcrate_provider_pattern_hour_cycleCoarseHourCycle {
() => {
// Module: crate::provider::pattern::hour_cycle
// Provides: {"CoarseHourCycle"}
// Dependencies: {}
# [doc = " Used to represent either H11/H12, or H23. Skeletons only store these"] # [doc = " hour cycles as H12 or H23."] # [derive (Debug , PartialEq , Clone , Copy , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: pattern))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [allow (clippy :: exhaustive_enums)] pub enum CoarseHourCycle { # [doc = " Can either be fields::Hour::H11 or fields::Hour::H12"] H11H12 , # [doc = " fields::Hour::H23"] H23 , }
};
}
