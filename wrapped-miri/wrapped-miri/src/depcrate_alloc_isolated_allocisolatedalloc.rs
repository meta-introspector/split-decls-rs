// Generated macro for IsolatedAlloc (struct)
macro_rules! Depcrate_alloc_isolated_allocIsolatedAlloc {
() => {
// Module: crate::alloc::isolated_alloc
// Provides: {"IsolatedAlloc"}
// Dependencies: {}
# [doc = " A dedicated allocator for interpreter memory contents, ensuring they are stored on dedicated"] # [doc = " pages (not mixed with Miri's own memory). This is used in native-lib mode."] # [derive (Debug)] pub struct IsolatedAlloc { # [doc = " Pointers to page-aligned memory that has been claimed by the allocator."] # [doc = " Every pointer here must point to a page-sized allocation claimed via"] # [doc = " mmap. These pointers are used for \"small\" allocations."] page_ptrs : Vec < NonNull < u8 > > , # [doc = " Metadata about which bytes have been allocated on each page. The length"] # [doc = " of this vector must be the same as that of `page_ptrs`, and the domain"] # [doc = " size of the bitset must be exactly `page_size / COMPRESSION_FACTOR`."] # [doc = ""] # [doc = " Conceptually, each bit of the bitset represents the allocation status of"] # [doc = " one n-byte chunk on the corresponding element of `page_ptrs`. Thus,"] # [doc = " indexing into it should be done with a value one-nth of the corresponding"] # [doc = " offset on the matching `page_ptrs` element (n = `COMPRESSION_FACTOR`)."] page_infos : Vec < DenseBitSet < usize > > , # [doc = " Pointers to multiple-page-sized allocations. These must also be page-aligned,"] # [doc = " with their size stored as the second element of the vector."] huge_ptrs : Vec < (NonNull < u8 > , usize) > , # [doc = " The host (not emulated) page size."] page_size : usize , }
};
}
