// Generated macro for DateTimeSeries (struct)
macro_rules! Depcrate_civil_datetimeDateTimeSeries {
() => {
// Module: crate::civil::datetime
// Provides: {"DateTimeSeries"}
// Dependencies: {}
# [doc = " An iterator over periodic datetimes, created by [`DateTime::series`]."] # [doc = ""] # [doc = " It is exhausted when the next value would exceed the limits of a [`Span`]"] # [doc = " or [`DateTime`] value."] # [doc = ""] # [doc = " This iterator is created by [`DateTime::series`]."] # [derive (Clone , Debug)] pub struct DateTimeSeries { start : DateTime , period : Span , step : i64 , }
};
}
