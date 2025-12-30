// Generated macro for total_memory_size (function)
macro_rules! Depcrate_mm_physicalmemtotal_memory_size {
() => {
// Module: crate::mm::physicalmem
// Provides: {"total_memory_size"}
// Dependencies: {}
pub fn total_memory_size () -> usize { TOTAL_MEMORY . load (Ordering :: Relaxed) }
};
}
