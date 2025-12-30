// Generated macro for CFBinaryHeapCompareContext (struct)
macro_rules! Depcrate_binary_heapCFBinaryHeapCompareContext {
() => {
// Module: crate::binary_heap
// Provides: {"CFBinaryHeapCompareContext"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFBinaryHeapCompareContext { pub version : CFIndex , pub info : * mut c_void , pub retain : extern "C" fn (info : * const c_void) -> * const c_void , pub release : extern "C" fn (info : * const c_void) , pub copyDescription : extern "C" fn (info : * const c_void) -> CFStringRef , }
};
}
