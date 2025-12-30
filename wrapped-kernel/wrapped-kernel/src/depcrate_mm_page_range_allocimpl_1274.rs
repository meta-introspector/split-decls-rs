// Generated macro for impl_1274 (impl)
macro_rules! Depcrate_mm_page_range_allocimpl_1274 {
() => {
// Module: crate::mm::page_range_alloc
// Provides: {"impl_1274"}
// Dependencies: {}
impl < A : PageRangeAllocator > PageRangeBox < A > { pub fn new (layout : PageLayout) -> Result < Self , AllocError > { let range = A :: allocate (layout) ? ; Ok (Self (range , PhantomData)) } pub unsafe fn from_raw (range : PageRange) -> Self { Self (range , PhantomData) } pub fn into_raw (b : Self) -> PageRange { let b = ManuallyDrop :: new (b) ; * * b } }
};
}
