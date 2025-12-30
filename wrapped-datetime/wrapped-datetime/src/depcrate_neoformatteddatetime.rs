// Generated macro for FormattedDateTime (struct)
macro_rules! Depcrate_neoFormattedDateTime {
() => {
// Module: crate::neo
// Provides: {"FormattedDateTime"}
// Dependencies: {}
# [doc = " An intermediate type during a datetime formatting operation."] # [doc = ""] # [doc = " Not intended to be stored: convert to a string first."] # [derive (Debug)] pub struct FormattedDateTime < 'a > { pattern : DateTimeZonePatternDataBorrowed < 'a > , input : DateTimeInputUnchecked , names : RawDateTimeNamesBorrowed < 'a > , }
};
}
