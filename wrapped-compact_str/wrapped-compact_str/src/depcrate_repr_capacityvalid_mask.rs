// Generated macro for VALID_MASK (const)
macro_rules! Depcrate_repr_capacityVALID_MASK {
() => {
// Module: crate::repr::capacity
// Provides: {"VALID_MASK"}
// Dependencies: {}
# [doc = " Mask of bits in [`Capacity`] that encode the value."] const VALID_MASK : usize = { let mut bytes = [255 ; USIZE_SIZE] ; bytes [USIZE_SIZE - 1] = 0 ; usize :: from_ne_bytes (bytes) } ;
};
}
