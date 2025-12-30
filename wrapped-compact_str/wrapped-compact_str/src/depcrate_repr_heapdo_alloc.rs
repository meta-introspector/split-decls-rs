// Generated macro for do_alloc (function)
macro_rules! Depcrate_repr_heapdo_alloc {
() => {
// Module: crate::repr::heap
// Provides: {"do_alloc"}
// Dependencies: {}
# [doc = " SAFETY: `layout` must not be zero sized"] # [inline] pub (crate) unsafe fn do_alloc (layout : Layout) -> Result < ptr :: NonNull < u8 > , ReserveError > { debug_assert ! (layout . size () > 0) ; let raw_ptr = :: alloc :: alloc :: alloc (layout) ; ptr :: NonNull :: new (raw_ptr) . ok_or (ReserveError (())) }
};
}
