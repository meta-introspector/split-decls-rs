// Generated macro for impl_1276 (impl)
macro_rules! Depcrate_mm_page_range_allocimpl_1276 {
() => {
// Module: crate::mm::page_range_alloc
// Provides: {"impl_1276"}
// Dependencies: {}
impl < A : PageRangeAllocator > Deref for PageRangeBox < A > { type Target = PageRange ; fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
