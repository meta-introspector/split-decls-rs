// Generated macro for other_95 (other)
macro_rules! Depcrate_binary_heapother_95 {
() => {
// Module: crate::binary_heap
// Provides: {"other_95"}
// Dependencies: {}
unsafe extern "C" { pub static kCFStringBinaryHeapCallBacks : CFBinaryHeapCallBacks ; pub fn CFBinaryHeapAddValue (heap : CFBinaryHeapRef , value : * const c_void) ; pub fn CFBinaryHeapApplyFunction (heap : CFBinaryHeapRef , applier : CFBinaryHeapApplierFunction , context : * mut c_void ,) ; pub fn CFBinaryHeapContainsValue (heap : CFBinaryHeapRef , value : * const c_void) -> Boolean ; pub fn CFBinaryHeapCreate (allocator : CFAllocatorRef , capacity : CFIndex , callBacks : * const CFBinaryHeapCallBacks , compareContext : * const CFBinaryHeapCompareContext ,) -> CFBinaryHeapRef ; pub fn CFBinaryHeapCreateCopy (allocator : CFAllocatorRef , capacity : CFIndex , heap : CFBinaryHeapRef ,) -> CFBinaryHeapRef ; pub fn CFBinaryHeapGetCount (heap : CFBinaryHeapRef) -> CFIndex ; pub fn CFBinaryHeapGetCountOfValue (heap : CFBinaryHeapRef , value : * const c_void) -> CFIndex ; pub fn CFBinaryHeapGetMinimum (heap : CFBinaryHeapRef) -> * const c_void ; pub fn CFBinaryHeapGetMinimumIfPresent (heap : CFBinaryHeapRef , value : * const * const c_void ,) -> Boolean ; pub fn CFBinaryHeapGetTypeID () -> CFTypeID ; pub fn CFBinaryHeapGetValues (heap : CFBinaryHeapRef , values : * const * const c_void) ; pub fn CFBinaryHeapRemoveAllValues (heap : CFBinaryHeapRef) ; pub fn CFBinaryHeapRemoveMinimumValue (heap : CFBinaryHeapRef) ; }
};
}
