// Generated macro for alloc_guard (function)
macro_rules! Depcrate_collections_raw_vecalloc_guard {
() => {
// Module: crate::collections::raw_vec
// Provides: {"alloc_guard"}
// Dependencies: {}
# [inline] fn alloc_guard (alloc_size : usize) -> Result < () , CollectionAllocErr > { if mem :: size_of :: < usize > () < 8 && alloc_size > :: core :: isize :: MAX as usize { Err (CapacityOverflow) } else { Ok (()) } }
};
}
