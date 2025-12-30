// Generated macro for UtcOffsetRecord (enum)
macro_rules! Depcrate_recordsUtcOffsetRecord {
() => {
// Module: crate::records
// Provides: {"UtcOffsetRecord"}
// Dependencies: {}
# [doc = " A `UtcOffsetRecord` that is either a minute precision or"] # [doc = " full precision UTC offset."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq)] pub enum UtcOffsetRecord { MinutePrecision (MinutePrecisionOffset) , FullPrecisionOffset (FullPrecisionOffset) , }
};
}
