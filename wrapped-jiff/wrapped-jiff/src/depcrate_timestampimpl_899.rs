// Generated macro for impl_899 (impl)
macro_rules! Depcrate_timestampimpl_899 {
() => {
// Module: crate::timestamp
// Provides: {"impl_899"}
// Dependencies: {}
impl TimestampSeries { # [inline] fn new (ts : Timestamp , period : Span) -> TimestampSeries { let duration = SignedDuration :: try_from (period) . ok () ; TimestampSeries { ts , duration } } }
};
}
