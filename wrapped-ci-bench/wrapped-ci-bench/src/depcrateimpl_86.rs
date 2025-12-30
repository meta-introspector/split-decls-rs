// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl CompareMemoryOperand { fn choose (& self , memory : MemoryDetails) -> u64 { match self { Self :: TotalBytes => memory . heap_total_bytes , Self :: TotalBlocks => memory . heap_total_blocks , Self :: PeakBytes => memory . heap_peak_bytes , Self :: PeakBlocks => memory . heap_peak_blocks , } } }
};
}
