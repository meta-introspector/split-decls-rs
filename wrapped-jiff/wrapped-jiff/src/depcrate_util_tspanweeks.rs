// Generated macro for SpanWeeks (type)
macro_rules! Depcrate_util_tSpanWeeks {
() => {
// Module: crate::util::t
// Provides: {"SpanWeeks"}
// Dependencies: {}
# [doc = " A range of the allowed number of weeks."] # [doc = ""] # [doc = " This is an upper bound and not actually a precise maximum. I believe a"] # [doc = " precise max could be fractional and not an integer."] pub (crate) type SpanWeeks = ri32 < { SpanDays :: MIN / 7 } , { SpanDays :: MAX / 7 } > ;
};
}
