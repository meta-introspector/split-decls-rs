// Generated macro for DurationParseRecord (struct)
macro_rules! Depcrate_recordsDurationParseRecord {
() => {
// Module: crate::records
// Provides: {"DurationParseRecord"}
// Dependencies: {}
# [doc = " The resulting record of parsing a `Duration` string."] # [allow (clippy :: exhaustive_structs)] # [cfg (feature = "duration")] # [derive (Debug , Clone , Copy , PartialEq)] pub struct DurationParseRecord { # [doc = " Duration Sign"] pub sign : Sign , # [doc = " The parsed `DateDurationRecord` if present."] pub date : Option < DateDurationRecord > , # [doc = " The parsed `TimeDurationRecord` if present."] pub time : Option < TimeDurationRecord > , }
};
}
