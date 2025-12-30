// Generated macro for DateDurationRecord (struct)
macro_rules! Depcrate_recordsDateDurationRecord {
() => {
// Module: crate::records
// Provides: {"DateDurationRecord"}
// Dependencies: {}
# [doc = " A `DateDurationRecord` represents the result of parsing the date component of a Duration string."] # [allow (clippy :: exhaustive_structs)] # [cfg (feature = "duration")] # [derive (Default , Debug , Clone , Copy , PartialEq)] pub struct DateDurationRecord { # [doc = " Years value."] pub years : u32 , # [doc = " Months value."] pub months : u32 , # [doc = " Weeks value."] pub weeks : u32 , # [doc = " Days value."] pub days : u64 , }
};
}
