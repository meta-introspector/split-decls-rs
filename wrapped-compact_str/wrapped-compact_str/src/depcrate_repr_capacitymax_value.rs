// Generated macro for MAX_VALUE (const)
macro_rules! Depcrate_repr_capacityMAX_VALUE {
() => {
// Module: crate::repr::capacity
// Provides: {"MAX_VALUE"}
// Dependencies: {}
# [doc = " The maximum value we're able to store, e.g. on 64-bit arch this is 2^56 - 2."] pub (crate) const MAX_VALUE : usize = { let mut bytes = [255 ; USIZE_SIZE] ; bytes [USIZE_SIZE - 1] = 0 ; usize :: from_le_bytes (bytes) - 1 } ;
};
}
