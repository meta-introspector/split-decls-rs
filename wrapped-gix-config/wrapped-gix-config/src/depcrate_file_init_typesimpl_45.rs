// Generated macro for impl_45 (impl)
macro_rules! Depcrate_file_init_typesimpl_45 {
() => {
// Module: crate::file::init::types
// Provides: {"impl_45"}
// Dependencies: {}
impl Options < '_ > { pub (crate) fn to_event_filter (self) -> Option < fn (& Event < '_ >) -> bool > { if self . lossy { Some (discard_nonessential_events) } else { None } } }
};
}
