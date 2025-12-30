// Generated macro for pack_size128 (function)
macro_rules! Depcratepack_size128 {
() => {
// Module: crate
// Provides: {"pack_size128"}
// Dependencies: {}
# [inline] fn pack_size128 (n : u128) -> usize { (16 - ((n | 1) . leading_zeros () >> 3)) as usize }
};
}
