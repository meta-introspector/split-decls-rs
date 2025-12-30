// Generated macro for impl_103 (impl)
macro_rules! Depcrate_compaction_filter_factoryimpl_103 {
() => {
// Module: crate::compaction_filter_factory
// Provides: {"impl_103"}
// Dependencies: {}
impl CompactionFilterContext { unsafe fn from_raw (ptr : * mut ffi :: rocksdb_compactionfiltercontext_t) -> Self { let is_full_compaction = unsafe { ffi :: rocksdb_compactionfiltercontext_is_full_compaction (ptr) } != 0 ; let is_manual_compaction = unsafe { ffi :: rocksdb_compactionfiltercontext_is_manual_compaction (ptr) } != 0 ; Self { is_full_compaction , is_manual_compaction , } } }
};
}
