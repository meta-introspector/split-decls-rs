// Generated macro for FormattedDateTimeUnchecked (struct)
macro_rules! Depcrate_uncheckedFormattedDateTimeUnchecked {
() => {
// Module: crate::unchecked
// Provides: {"FormattedDateTimeUnchecked"}
// Dependencies: {}
# [doc = " An intermediate type during a datetime formatting operation with dynamic input."] # [doc = ""] # [doc = " Unlike [`FormattedDateTime`], converting this to a string could fail."] # [doc = ""] # [doc = " Not intended to be stored: convert to a string first."] # [derive (Debug)] pub struct FormattedDateTimeUnchecked < 'a > { pattern : DateTimeZonePatternDataBorrowed < 'a > , input : DateTimeInputUnchecked , names : RawDateTimeNamesBorrowed < 'a > , }
};
}
