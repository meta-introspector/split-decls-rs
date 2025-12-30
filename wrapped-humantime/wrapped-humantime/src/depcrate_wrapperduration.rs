// Generated macro for Duration (struct)
macro_rules! Depcrate_wrapperDuration {
() => {
// Module: crate::wrapper
// Provides: {"Duration"}
// Dependencies: {}
# [doc = " A wrapper for duration that has `FromStr` implementation"] # [doc = ""] # [doc = " This is useful if you want to use it somewhere where `FromStr` is"] # [doc = " expected."] # [doc = ""] # [doc = " See `parse_duration` for the description of the format."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = " let x: Duration;"] # [doc = " x = \"12h 5min 2ns\".parse::<humantime::Duration>().unwrap().into();"] # [doc = " assert_eq!(x, Duration::new(12*3600 + 5*60, 2))"] # [doc = " ```"] # [doc = ""] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Default)] pub struct Duration (StdDuration) ;
};
}
