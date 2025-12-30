// Generated macro for impl_1312 (impl)
macro_rules! Depcrate_mm_virtualmemimpl_1312 {
() => {
// Module: crate::mm::virtualmem
// Provides: {"impl_1312"}
// Dependencies: {}
impl PageRangeAllocator for PageAlloc { unsafe fn init () { unsafe { init () ; } } fn allocate (layout : PageLayout) -> Result < PageRange , AllocError > { KERNEL_FREE_LIST . lock () . allocate (layout) . map_err (| _ | AllocError) } fn allocate_at (range : PageRange) -> Result < () , AllocError > { KERNEL_FREE_LIST . lock () . allocate_at (range) . map_err (| _ | AllocError) } unsafe fn deallocate (range : PageRange) { unsafe { KERNEL_FREE_LIST . lock () . deallocate (range) . unwrap () ; } } }
};
}
