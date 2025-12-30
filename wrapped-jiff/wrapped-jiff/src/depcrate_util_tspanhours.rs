// Generated macro for SpanHours (type)
macro_rules! Depcrate_util_tSpanHours {
() => {
// Module: crate::util::t
// Provides: {"SpanHours"}
// Dependencies: {}
# [doc = " A range of the allowed number of hours."] # [doc = ""] # [doc = " Like days, this is an upper bound because some days (because of DST) have"] # [doc = " 25 hours."] pub (crate) type SpanHours = ri32 < { SpanMinutes :: MIN / 60 } , { SpanMinutes :: MAX / 60 } > ;
};
}
