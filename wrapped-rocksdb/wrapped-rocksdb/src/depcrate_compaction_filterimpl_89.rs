// Generated macro for impl_89 (impl)
macro_rules! Depcrate_compaction_filterimpl_89 {
() => {
// Module: crate::compaction_filter
// Provides: {"impl_89"}
// Dependencies: {}
impl < F > CompactionFilter for CompactionFilterCallback < F > where F : CompactionFilterFn , { fn name (& self) -> & CStr { self . name . as_c_str () } fn filter (& mut self , level : u32 , key : & [u8] , value : & [u8]) -> Decision { (self . filter_fn) (level , key , value) } }
};
}
