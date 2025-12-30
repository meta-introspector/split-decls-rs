// Generated macro for impl_1275 (impl)
macro_rules! Depcrate_mm_page_range_allocimpl_1275 {
() => {
// Module: crate::mm::page_range_alloc
// Provides: {"impl_1275"}
// Dependencies: {}
impl < A : PageRangeAllocator > Drop for PageRangeBox < A > { fn drop (& mut self) { unsafe { A :: deallocate (self . 0) ; } } }
};
}
