// Generated macro for MIN_HEAP_SIZE (const)
macro_rules! Depcrate_repr_heapMIN_HEAP_SIZE {
() => {
// Module: crate::repr::heap
// Provides: {"MIN_HEAP_SIZE"}
// Dependencies: {}
# [doc = " The minimum size we'll allocate on the heap is one usize larger than our max inline size"] const MIN_HEAP_SIZE : usize = MAX_SIZE + mem :: size_of :: < usize > () ;
};
}
