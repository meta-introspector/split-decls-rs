// Generated macro for SpanSecondsOrLowerNanoseconds (type)
macro_rules! Depcrate_util_tSpanSecondsOrLowerNanoseconds {
() => {
// Module: crate::util::t
// Provides: {"SpanSecondsOrLowerNanoseconds"}
// Dependencies: {}
# [doc = " The range of allowable seconds and lower in a span, in units of"] # [doc = " nanoseconds."] # [doc = ""] # [doc = " See `SpanSecondsOrLower`. This exists for the same reason. Namely, when"] # [doc = " serializing a `Span` to an ISO 8601 duration string, we need to combine"] # [doc = " seconds and lower into a single fractional seconds value."] pub (crate) type SpanSecondsOrLowerNanoseconds = ri128 < { (SpanSeconds :: MIN * NANOS_PER_SECOND . bound ()) + (SpanMilliseconds :: MIN * NANOS_PER_MILLI . bound ()) + (SpanMicroseconds :: MIN * NANOS_PER_MICRO . bound ()) + SpanNanoseconds :: MIN } , { (SpanSeconds :: MAX * NANOS_PER_SECOND . bound ()) + (SpanMilliseconds :: MAX * NANOS_PER_MILLI . bound ()) + (SpanMicroseconds :: MAX * NANOS_PER_MICRO . bound ()) + SpanNanoseconds :: MAX } , > ;
};
}
