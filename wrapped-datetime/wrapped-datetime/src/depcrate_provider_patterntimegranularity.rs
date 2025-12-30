// Generated macro for TimeGranularity (enum)
macro_rules! Depcrate_provider_patternTimeGranularity {
() => {
// Module: crate::provider::pattern
// Provides: {"TimeGranularity"}
// Dependencies: {}
# [doc = " The granularity of time represented in a [`Pattern`](runtime::Pattern)."] # [doc = " Ordered from least granular to most granular for comparison."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , yoke :: Yokeable , zerofrom :: ZeroFrom ,)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: pattern))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [non_exhaustive] pub enum TimeGranularity { # [doc = " No time is in the pattern."] None , # [doc = " Smallest time unit = hours."] Hours , # [doc = " Smallest time unit = minutes."] Minutes , # [doc = " Smallest time unit = seconds."] Seconds , # [doc = " Smallest time unit = Nanoseconds."] Nanoseconds , }
};
}
