// Generated macro for impl_93 (impl)
macro_rules! Depcrate_collectorimpl_93 {
() => {
// Module: crate::collector
// Provides: {"impl_93"}
// Dependencies: {}
impl PartialEq for Collector { # [doc = " Checks if both handles point to the same collector."] fn eq (& self , rhs : & Self) -> bool { Arc :: ptr_eq (& self . global , & rhs . global) } }
};
}
