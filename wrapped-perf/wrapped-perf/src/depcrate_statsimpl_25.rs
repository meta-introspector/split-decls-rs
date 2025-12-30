// Generated macro for impl_25 (impl)
macro_rules! Depcrate_statsimpl_25 {
() => {
// Module: crate::stats
// Provides: {"impl_25"}
// Dependencies: {}
impl Interval { fn new (start : Duration , end : Duration) -> Self { let period = IntervalPeriod { start : start . as_secs_f64 () , end : end . as_secs_f64 () , seconds : (end - start) . as_secs_f64 () , } ; Self { streams : vec ! [] , period , } } fn record_stream_stats (& mut self , stream_stats : Arc < StreamStats >) { let bytes = stream_stats . bytes . swap (0 , Ordering :: SeqCst) ; self . streams . push (StreamIntervalStats { id : stream_stats . id , bytes , sender : stream_stats . sender , }) } }
};
}
