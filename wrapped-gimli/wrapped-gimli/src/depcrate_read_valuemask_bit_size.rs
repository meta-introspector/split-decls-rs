// Generated macro for mask_bit_size (function)
macro_rules! Depcrate_read_valuemask_bit_size {
() => {
// Module: crate::read::value
// Provides: {"mask_bit_size"}
// Dependencies: {}
# [inline] fn mask_bit_size (addr_mask : u64) -> u32 { 64 - addr_mask . leading_zeros () }
};
}
