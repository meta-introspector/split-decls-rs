// Generated macro for CAPACITY_IS_ON_THE_HEAP (const)
macro_rules! Depcrate_repr_capacityCAPACITY_IS_ON_THE_HEAP {
() => {
// Module: crate::repr::capacity
// Provides: {"CAPACITY_IS_ON_THE_HEAP"}
// Dependencies: {}
# [doc = " State that describes the capacity as being stored on the heap."] # [doc = ""] # [doc = " All bytes `255`, with the last being [`LastByte::Heap`], using the same amount of bytes"] # [doc = " as `usize`. Example (64-bit): `[255, 255, 255, 255, 255, 255, 255, 216]`"] # [cfg (not (target_pointer_width = "64"))] const CAPACITY_IS_ON_THE_HEAP : Capacity = Capacity (VALID_MASK | HEAP_MARKER) ;
};
}
