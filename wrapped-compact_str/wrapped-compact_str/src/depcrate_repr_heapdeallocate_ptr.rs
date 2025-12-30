// Generated macro for deallocate_ptr (function)
macro_rules! Depcrate_repr_heapdeallocate_ptr {
() => {
// Module: crate::repr::heap
// Provides: {"deallocate_ptr"}
// Dependencies: {}
# [doc = " Deallocates a buffer on the heap, handling when the capacity is also stored on the heap"] # [inline] pub (crate) fn deallocate_ptr (ptr : ptr :: NonNull < u8 > , cap : Capacity) { # [cold] fn deallocate_with_capacity_on_heap (ptr : ptr :: NonNull < u8 >) { let adj_ptr = ptr . as_ptr () . wrapping_sub (mem :: size_of :: < usize > ()) ; let mut buf = [0u8 ; mem :: size_of :: < usize > ()] ; unsafe { ptr :: copy_nonoverlapping (adj_ptr , buf . as_mut_ptr () , mem :: size_of :: < usize > ()) ; } let capacity = usize :: from_ne_bytes (buf) ; let ptr = unsafe { ptr :: NonNull :: new_unchecked (adj_ptr) } ; unsafe { heap_capacity :: dealloc (ptr , capacity) } } if cap . is_heap () { deallocate_with_capacity_on_heap (ptr) ; } else { unsafe { inline_capacity :: dealloc (ptr , cap . as_usize ()) } } }
};
}
