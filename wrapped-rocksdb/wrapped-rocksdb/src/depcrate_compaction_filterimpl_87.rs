// Generated macro for impl_87 (impl)
macro_rules! Depcrate_compaction_filterimpl_87 {
() => {
// Module: crate::compaction_filter
// Provides: {"impl_87"}
// Dependencies: {}
impl < F > CompactionFilterFn for F where F : FnMut (u32 , & [u8] , & [u8]) -> Decision + Send + 'static { }
};
}
