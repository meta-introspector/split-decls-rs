// Generated macro for PageRangeAllocator (trait)
macro_rules! Depcrate_mm_page_range_allocPageRangeAllocator {
() => {
// Module: crate::mm::page_range_alloc
// Provides: {"PageRangeAllocator"}
// Dependencies: {}
# [doc = " An allocator that allocates memory in page granularity."] pub trait PageRangeAllocator { unsafe fn init () ; # [doc = " Attempts to allocate a range of memory in page granularity."] fn allocate (layout : PageLayout) -> Result < PageRange , AllocError > ; # [doc = " Attempts to allocate the pages described by `range`."] fn allocate_at (range : PageRange) -> Result < () , AllocError > ; # [doc = " Deallocates the pages described by `range`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `range` must described a range of pages _currently allocated_ via this allocator."] unsafe fn deallocate (range : PageRange) ; }
};
}
