// Generated macro for SpanSecondsOrLower (type)
macro_rules! Depcrate_util_tSpanSecondsOrLower {
() => {
// Module: crate::util::t
// Provides: {"SpanSecondsOrLower"}
// Dependencies: {}
# [doc = " The range of allowable seconds and lower in a span, in units of seconds."] # [doc = ""] # [doc = " This corresponds to when the min/max of seconds, milliseconds, microseconds"] # [doc = " and nanoseconds are added together in a span. This is useful for describing"] # [doc = " the limit on the total number of possible seconds when all of these units"] # [doc = " are combined. This is necessary as part of printing/parsing spans because"] # [doc = " the ISO 8601 duration format doesn't support individual millisecond,"] # [doc = " microsecond and nanosecond components. So they all need to be smushed into"] # [doc = " seconds and a possible fractional part."] pub (crate) type SpanSecondsOrLower = ri64 < { SpanSeconds :: MIN + (SpanMilliseconds :: MIN / MILLIS_PER_SECOND . bound ()) + (SpanMicroseconds :: MIN / MICROS_PER_SECOND . bound ()) + (SpanNanoseconds :: MIN / NANOS_PER_SECOND . bound ()) } , { SpanSeconds :: MAX + (SpanMilliseconds :: MAX / MILLIS_PER_SECOND . bound ()) + (SpanMicroseconds :: MAX / MICROS_PER_SECOND . bound ()) + (SpanNanoseconds :: MAX / NANOS_PER_SECOND . bound ()) } , > ;
};
}
