// Generated macro for original_capacity_to_repr (function)
macro_rules! Depcrate_bytes_mutoriginal_capacity_to_repr {
() => {
// Module: crate::bytes_mut
// Provides: {"original_capacity_to_repr"}
// Dependencies: {}
# [inline] fn original_capacity_to_repr (cap : usize) -> usize { let width = PTR_WIDTH - ((cap >> MIN_ORIGINAL_CAPACITY_WIDTH) . leading_zeros () as usize) ; cmp :: min (width , MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH ,) }
};
}
