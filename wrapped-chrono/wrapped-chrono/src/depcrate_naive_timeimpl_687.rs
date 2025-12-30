// Generated macro for impl_687 (impl)
macro_rules! Depcrate_naive_timeimpl_687 {
() => {
// Module: crate::naive::time
// Provides: {"impl_687"}
// Dependencies: {}
# [doc = " The default value for a NaiveTime is midnight, 00:00:00 exactly."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use chrono::NaiveTime;"] # [doc = ""] # [doc = " let default_time = NaiveTime::default();"] # [doc = " assert_eq!(default_time, NaiveTime::from_hms_opt(0, 0, 0).unwrap());"] # [doc = " ```"] impl Default for NaiveTime { fn default () -> Self { NaiveTime :: from_hms_opt (0 , 0 , 0) . unwrap () } }
};
}
