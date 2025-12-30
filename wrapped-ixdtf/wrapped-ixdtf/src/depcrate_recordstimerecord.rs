// Generated macro for TimeRecord (struct)
macro_rules! Depcrate_recordsTimeRecord {
() => {
// Module: crate::records
// Provides: {"TimeRecord"}
// Dependencies: {}
# [doc = " Parsed Time info"] # [allow (clippy :: exhaustive_structs)] # [derive (Debug , Default , Clone , Copy , PartialEq)] pub struct TimeRecord { # [doc = " An hour"] pub hour : u8 , # [doc = " A minute value"] pub minute : u8 , # [doc = " A second value."] pub second : u8 , # [doc = " A nanosecond value representing all sub-second components."] pub fraction : Option < Fraction > , }
};
}
