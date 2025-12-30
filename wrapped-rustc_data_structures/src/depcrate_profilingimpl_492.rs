// Generated macro for impl_492 (impl)
macro_rules! Depcrate_profilingimpl_492 {
() => {
// Module: crate::profiling
// Provides: {"impl_492"}
// Dependencies: {}
impl Drop for VerboseTimingGuard < '_ > { fn drop (& mut self) { if let Some (info) = & self . info { let end_rss = get_resident_set_size () ; let dur = info . start_time . elapsed () ; print_time_passes_entry (& info . message , dur , info . start_rss , end_rss , info . format) ; } } }
};
}
