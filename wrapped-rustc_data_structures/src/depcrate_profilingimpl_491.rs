// Generated macro for impl_491 (impl)
macro_rules! Depcrate_profilingimpl_491 {
() => {
// Module: crate::profiling
// Provides: {"impl_491"}
// Dependencies: {}
impl < 'a > VerboseTimingGuard < 'a > { pub fn start (message_and_format : Option < (String , TimePassesFormat) > , _guard : TimingGuard < 'a > ,) -> Self { VerboseTimingGuard { _guard , info : message_and_format . map (| (message , format) | VerboseInfo { start_time : Instant :: now () , start_rss : get_resident_set_size () , message , format , }) , } } # [inline (always)] pub fn run < R > (self , f : impl FnOnce () -> R) -> R { let _timer = self ; f () } }
};
}
