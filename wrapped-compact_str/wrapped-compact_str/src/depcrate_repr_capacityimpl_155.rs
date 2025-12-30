// Generated macro for impl_155 (impl)
macro_rules! Depcrate_repr_capacityimpl_155 {
() => {
// Module: crate::repr::capacity
// Provides: {"impl_155"}
// Dependencies: {}
impl Capacity { # [inline] pub (crate) const fn new (capacity : usize) -> Self { cfg_if :: cfg_if ! { if # [cfg (target_pointer_width = "64")] { debug_assert ! (capacity <= MAX_VALUE) ; Capacity (capacity . to_le () | HEAP_MARKER) } else if # [cfg (target_pointer_width = "32")] { if capacity > MAX_VALUE { CAPACITY_IS_ON_THE_HEAP } else { Capacity (capacity . to_le () | HEAP_MARKER) } } else { compile_error ! ("Unsupported target_pointer_width") ; } } } # [doc = " Re-interprets a [`Capacity`] as a `usize`"] # [doc = ""] # [doc = " # SAFETY:"] # [doc = " * `self` must be less than or equal to [`MAX_VALUE`]"] # [inline (always)] pub (crate) unsafe fn as_usize (self) -> usize { usize :: from_le (self . 0 & VALID_MASK) } # [doc = " Returns whether or not this [`Capacity`] has a value that indicates the capacity is being"] # [doc = " stored on the heap"] # [inline (always)] pub (crate) fn is_heap (self) -> bool { cfg_if :: cfg_if ! { if # [cfg (target_pointer_width = "64")] { false } else { self == CAPACITY_IS_ON_THE_HEAP } } } }
};
}
