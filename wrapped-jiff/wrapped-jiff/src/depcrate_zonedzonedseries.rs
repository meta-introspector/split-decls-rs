// Generated macro for ZonedSeries (struct)
macro_rules! Depcrate_zonedZonedSeries {
() => {
// Module: crate::zoned
// Provides: {"ZonedSeries"}
// Dependencies: {}
# [doc = " An iterator over periodic zoned datetimes, created by [`Zoned::series`]."] # [doc = ""] # [doc = " It is exhausted when the next value would exceed the limits of a [`Span`]"] # [doc = " or [`Zoned`] value."] # [doc = ""] # [doc = " This iterator is created by [`Zoned::series`]."] # [derive (Clone , Debug)] pub struct ZonedSeries { start : Zoned , prev : Option < Timestamp > , period : Span , step : i64 , }
};
}
