// Generated macro for Timestamp (struct)
macro_rules! Depcrate_wrapperTimestamp {
() => {
// Module: crate::wrapper
// Provides: {"Timestamp"}
// Dependencies: {}
# [doc = " A wrapper for SystemTime that has `FromStr` implementation"] # [doc = ""] # [doc = " This is useful if you want to use it somewhere where `FromStr` is"] # [doc = " expected."] # [doc = ""] # [doc = " See `parse_rfc3339_weak` for the description of the format. The \"weak\""] # [doc = " format is used as it's more pemissive for human input as this is the"] # [doc = " expected use of the type (e.g. command-line parsing)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::SystemTime;"] # [doc = " let x: SystemTime;"] # [doc = " x = \"2018-02-16T00:31:37Z\".parse::<humantime::Timestamp>().unwrap().into();"] # [doc = " assert_eq!(humantime::format_rfc3339(x).to_string(), \"2018-02-16T00:31:37Z\");"] # [doc = " ```"] # [doc = ""] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Timestamp (SystemTime) ;
};
}
