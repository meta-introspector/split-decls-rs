// Generated macro for CompactionFilterCallback (struct)
macro_rules! Depcrate_compaction_filterCompactionFilterCallback {
() => {
// Module: crate::compaction_filter
// Provides: {"CompactionFilterCallback"}
// Dependencies: {}
pub struct CompactionFilterCallback < F > where F : CompactionFilterFn , { pub name : CString , pub filter_fn : F , }
};
}
