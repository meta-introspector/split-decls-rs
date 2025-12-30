// Generated macro for TimeDurationRecord (enum)
macro_rules! Depcrate_recordsTimeDurationRecord {
() => {
// Module: crate::records
// Provides: {"TimeDurationRecord"}
// Dependencies: {}
# [doc = " A `TimeDurationRecord` represents the result of parsing the time component of a Duration string."] # [allow (clippy :: exhaustive_enums)] # [cfg (feature = "duration")] # [derive (Debug , Clone , Copy , PartialEq)] pub enum TimeDurationRecord { Hours { # [doc = " Hours value."] hours : u64 , # [doc = " The parsed fractional digits."] fraction : Option < Fraction > , } , Minutes { # [doc = " Hours value."] hours : u64 , # [doc = " Minutes value."] minutes : u64 , # [doc = " The parsed fractional digits."] fraction : Option < Fraction > , } , Seconds { # [doc = " Hours value."] hours : u64 , # [doc = " Minutes value."] minutes : u64 , # [doc = " Seconds value."] seconds : u64 , # [doc = " The parsed fractional digits."] fraction : Option < Fraction > , } , }
};
}
