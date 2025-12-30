// Generated macro for DateSeries (struct)
macro_rules! Depcrate_civil_dateDateSeries {
() => {
// Module: crate::civil::date
// Provides: {"DateSeries"}
// Dependencies: {}
# [doc = " An iterator over periodic dates, created by [`Date::series`]."] # [doc = ""] # [doc = " It is exhausted when the next value would exceed the limits of a [`Span`]"] # [doc = " or [`Date`] value."] # [doc = ""] # [doc = " This iterator is created by [`Date::series`]."] # [derive (Clone , Debug)] pub struct DateSeries { start : Date , period : Span , step : i64 , }
};
}
