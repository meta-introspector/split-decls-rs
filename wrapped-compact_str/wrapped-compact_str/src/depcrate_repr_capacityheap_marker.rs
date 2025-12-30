// Generated macro for HEAP_MARKER (const)
macro_rules! Depcrate_repr_capacityHEAP_MARKER {
() => {
// Module: crate::repr::capacity
// Provides: {"HEAP_MARKER"}
// Dependencies: {}
# [doc = " Mask of bits that are set in [`Capacity`] if the string data is stored on the heap."] const HEAP_MARKER : usize = { let mut bytes = [0 ; USIZE_SIZE] ; bytes [USIZE_SIZE - 1] = LastByte :: Heap as u8 ; usize :: from_ne_bytes (bytes) } ;
};
}
