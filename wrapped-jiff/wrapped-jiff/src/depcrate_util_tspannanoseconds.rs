// Generated macro for SpanNanoseconds (type)
macro_rules! Depcrate_util_tSpanNanoseconds {
() => {
// Module: crate::util::t
// Provides: {"SpanNanoseconds"}
// Dependencies: {}
# [doc = " A range of the allowed number of nanoseconds."] # [doc = ""] # [doc = " For this, we cannot cover the full span of supported time instants since"] # [doc = " `UnixSeconds::MAX * NANOSECONDS_PER_SECOND` cannot fit into 64-bits. We"] # [doc = " could use a `i128`, but it doesn't seem worth it."] # [doc = ""] # [doc = " Also note that our min is equal to -max, so that the total number of values"] # [doc = " in this range is one less than the number of distinct `i64` values. We do"] # [doc = " that so that the absolute value is always defined."] pub (crate) type SpanNanoseconds = ri64 < { (i64 :: MIN + 1) as i128 } , { i64 :: MAX as i128 } > ;
};
}
