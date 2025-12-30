// Generated macro for isolated_alloc (module)
macro_rules! Depcrate_allocisolated_alloc {
() => {
// Module: crate::alloc
// Provides: {"isolated_alloc"}
// Dependencies: {}
# [cfg (not (all (unix , feature = "native-lib")))] pub mod isolated_alloc { use std :: alloc :: Layout ; # [doc = " Stub allocator to avoid `cfg`s in the rest of Miri."] # [derive (Debug)] pub struct IsolatedAlloc (!) ; impl IsolatedAlloc { pub fn new () -> Self { unreachable ! () } pub unsafe fn alloc (& mut self , _layout : Layout) -> * mut u8 { match self . 0 { } } pub unsafe fn alloc_zeroed (& mut self , _layout : Layout) -> * mut u8 { match self . 0 { } } pub unsafe fn dealloc (& mut self , _ptr : * mut u8 , _layout : Layout) { match self . 0 { } } } }
};
}
