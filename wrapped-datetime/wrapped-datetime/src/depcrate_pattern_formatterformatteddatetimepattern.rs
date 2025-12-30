// Generated macro for FormattedDateTimePattern (struct)
macro_rules! Depcrate_pattern_formatterFormattedDateTimePattern {
() => {
// Module: crate::pattern::formatter
// Provides: {"FormattedDateTimePattern"}
// Dependencies: {}
# [doc = " A pattern that has been interpolated and implements [`TryWriteable`]."] # [derive (Debug)] pub struct FormattedDateTimePattern < 'a > { pattern : DateTimePatternBorrowed < 'a > , input : DateTimeInputUnchecked , names : RawDateTimeNamesBorrowed < 'a > , }
};
}
