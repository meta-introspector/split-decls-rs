// Generated macro for SpanMonths (type)
macro_rules! Depcrate_util_tSpanMonths {
() => {
// Module: crate::util::t
// Provides: {"SpanMonths"}
// Dependencies: {}
# [doc = " A precise min/max of the allowed range of a duration in months."] pub (crate) type SpanMonths = ri32 < { SpanYears :: MIN * MONTHS_PER_YEAR . bound () } , { SpanYears :: MAX * MONTHS_PER_YEAR . bound () } , > ;
};
}
