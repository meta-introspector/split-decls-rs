// Generated macro for TimestampSeries (struct)
macro_rules! Depcrate_timestampTimestampSeries {
() => {
// Module: crate::timestamp
// Provides: {"TimestampSeries"}
// Dependencies: {}
# [doc = " An iterator over periodic timestamps, created by [`Timestamp::series`]."] # [doc = ""] # [doc = " It is exhausted when the next value would exceed the limits of a [`Span`]"] # [doc = " or [`Timestamp`] value."] # [doc = ""] # [doc = " This iterator is created by [`Timestamp::series`]."] # [derive (Clone , Debug)] pub struct TimestampSeries { ts : Timestamp , duration : Option < SignedDuration > , }
};
}
