// Generated macro for allocate_ptr (function)
macro_rules! Depcrate_repr_heapallocate_ptr {
() => {
// Module: crate::repr::heap
// Provides: {"allocate_ptr"}
// Dependencies: {}
# [doc = " Allocates a buffer on the heap that we can use to store a string, optionally stores the capacity"] # [doc = " of said buffer on the heap."] # [doc = ""] # [doc = " Returns a [`Capacity`] that either indicates the capacity is stored on the heap, or is stored"] # [doc = " in the `Capacity` itself."] # [inline] pub (crate) fn allocate_ptr (capacity : usize) -> Result < (Capacity , ptr :: NonNull < u8 >) , ReserveError > { let capacity = capacity . max (MIN_HEAP_SIZE) ; let cap = Capacity :: new (capacity) ; debug_assert ! (capacity > 0) ; # [cold] fn allocate_with_capacity_on_heap (capacity : usize) -> Result < ptr :: NonNull < u8 > , ReserveError > { let ptr = unsafe { heap_capacity :: alloc (capacity) ? } ; unsafe { ptr :: copy_nonoverlapping (capacity . to_ne_bytes () . as_ptr () , ptr . as_ptr () , mem :: size_of :: < usize > () ,) } ; let raw_ptr = ptr . as_ptr () . wrapping_add (core :: mem :: size_of :: < usize > ()) ; Ok (unsafe { ptr :: NonNull :: new_unchecked (raw_ptr) }) } let ptr = if cap . is_heap () { allocate_with_capacity_on_heap (capacity) } else { unsafe { inline_capacity :: alloc (capacity) } } ; Ok ((cap , ptr ?)) }
};
}
