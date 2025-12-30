// Generated macro for DateTimeRecord (struct)
macro_rules! Depcrate_parsers_datetimeDateTimeRecord {
() => {
// Module: crate::parsers::datetime
// Provides: {"DateTimeRecord"}
// Dependencies: {}
# [derive (Debug , Default , Clone)] # [doc = " A `DateTime` Parse Node that contains the date, time, and offset info."] pub (crate) struct DateTimeRecord { # [doc = " Date"] pub (crate) date : Option < DateRecord > , # [doc = " Time"] pub (crate) time : Option < TimeRecord > , # [doc = " Tz Offset"] pub (crate) time_zone : Option < UtcOffsetRecordOrZ > , }
};
}
