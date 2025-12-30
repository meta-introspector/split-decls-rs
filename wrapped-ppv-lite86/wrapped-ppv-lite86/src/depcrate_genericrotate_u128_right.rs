// Generated macro for rotate_u128_right (function)
macro_rules! Depcrate_genericrotate_u128_right {
() => {
// Module: crate::generic
// Provides: {"rotate_u128_right"}
// Dependencies: {}
# [inline (always)] fn rotate_u128_right (x : u128 , i : u32) -> u128 { (x >> i) | (x << (128 - i)) }
};
}
