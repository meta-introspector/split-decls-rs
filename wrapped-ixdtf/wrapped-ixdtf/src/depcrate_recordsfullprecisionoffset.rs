// Generated macro for FullPrecisionOffset (struct)
macro_rules! Depcrate_recordsFullPrecisionOffset {
() => {
// Module: crate::records
// Provides: {"FullPrecisionOffset"}
// Dependencies: {}
# [doc = " A full precision UTC offset represented by a `MinutePrecisionOffset`"] # [doc = " with seconds and an optional fractional seconds"] # [derive (Debug , Clone , Copy , PartialEq)] # [allow (clippy :: exhaustive_structs)] pub struct FullPrecisionOffset { # [doc = " The minute precision offset of a `FullPrecisionOffset`."] pub minute_precision_offset : MinutePrecisionOffset , # [doc = " The second value of a `FullPrecisionOffset`."] pub second : u8 , # [doc = " Any nanosecond value of a `FullPrecisionOffset`."] pub fraction : Option < Fraction > , }
};
}
