// Generated macro for impl_29 (impl)
macro_rules! Depcrate_filtered_logimpl_29 {
() => {
// Module: crate::filtered_log
// Provides: {"impl_29"}
// Dependencies: {}
impl < T : Log > FilteredLog < T > { # [doc = " Create a new filtered log."] pub fn new (log : T , filter : Filter) -> Self { Self { log , filter } } }
};
}
