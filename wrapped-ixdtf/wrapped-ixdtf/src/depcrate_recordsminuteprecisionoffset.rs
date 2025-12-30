// Generated macro for MinutePrecisionOffset (struct)
macro_rules! Depcrate_recordsMinutePrecisionOffset {
() => {
// Module: crate::records
// Provides: {"MinutePrecisionOffset"}
// Dependencies: {}
# [doc = " A minute preicision UTC offset"] # [derive (Debug , Clone , Copy , PartialEq)] # [allow (clippy :: exhaustive_structs)] pub struct MinutePrecisionOffset { # [doc = " The `Sign` value of the `UtcOffsetRecord`."] pub sign : Sign , # [doc = " The hour value of the `UtcOffsetRecord`."] pub hour : u8 , # [doc = " The minute value of the `UtcOffsetRecord`."] pub minute : u8 , }
};
}
