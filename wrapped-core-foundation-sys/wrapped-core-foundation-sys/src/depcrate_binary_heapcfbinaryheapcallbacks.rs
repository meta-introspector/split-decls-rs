// Generated macro for CFBinaryHeapCallBacks (struct)
macro_rules! Depcrate_binary_heapCFBinaryHeapCallBacks {
() => {
// Module: crate::binary_heap
// Provides: {"CFBinaryHeapCallBacks"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFBinaryHeapCallBacks { pub version : CFIndex , pub retain : extern "C" fn (allocator : CFAllocatorRef , ptr : * const c_void) -> * const c_void , pub release : extern "C" fn (allocator : CFAllocatorRef , ptr : * const c_void) , pub copyDescription : extern "C" fn (ptr : * const c_void) -> CFStringRef , pub compare : extern "C" fn (ptr1 : * const c_void , ptr2 : * const c_void , context : * mut c_void ,) -> CFComparisonResult , }
};
}
