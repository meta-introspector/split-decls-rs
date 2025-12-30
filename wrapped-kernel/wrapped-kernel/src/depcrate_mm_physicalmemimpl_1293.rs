// Generated macro for impl_1293 (impl)
macro_rules! Depcrate_mm_physicalmemimpl_1293 {
() => {
// Module: crate::mm::physicalmem
// Provides: {"impl_1293"}
// Dependencies: {}
impl PageRangeAllocator for FrameAlloc { unsafe fn init () { unsafe { init () ; } } fn allocate (layout : PageLayout) -> Result < PageRange , AllocError > { PHYSICAL_FREE_LIST . lock () . allocate (layout) . map_err (| _ | AllocError) } fn allocate_at (range : PageRange) -> Result < () , AllocError > { PHYSICAL_FREE_LIST . lock () . allocate_at (range) . map_err (| _ | AllocError) } unsafe fn deallocate (range : PageRange) { unsafe { PHYSICAL_FREE_LIST . lock () . deallocate (range) . unwrap () ; } } }
};
}
