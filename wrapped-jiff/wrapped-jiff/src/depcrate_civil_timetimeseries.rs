// Generated macro for TimeSeries (struct)
macro_rules! Depcrate_civil_timeTimeSeries {
() => {
// Module: crate::civil::time
// Provides: {"TimeSeries"}
// Dependencies: {}
# [doc = " An iterator over periodic times, created by [`Time::series`]."] # [doc = ""] # [doc = " It is exhausted when the next value would exceed the limits of a [`Span`]"] # [doc = " or [`Time`] value."] # [doc = ""] # [doc = " This iterator is created by [`Time::series`]."] # [derive (Clone , Debug)] pub struct TimeSeries { start : Time , period : Span , step : i64 , }
};
}
