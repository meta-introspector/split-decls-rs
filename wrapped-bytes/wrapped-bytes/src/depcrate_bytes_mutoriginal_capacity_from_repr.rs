// Generated macro for original_capacity_from_repr (function)
macro_rules! Depcrate_bytes_mutoriginal_capacity_from_repr {
() => {
// Module: crate::bytes_mut
// Provides: {"original_capacity_from_repr"}
// Dependencies: {}
fn original_capacity_from_repr (repr : usize) -> usize { if repr == 0 { return 0 ; } 1 << (repr + (MIN_ORIGINAL_CAPACITY_WIDTH - 1)) }
};
}
