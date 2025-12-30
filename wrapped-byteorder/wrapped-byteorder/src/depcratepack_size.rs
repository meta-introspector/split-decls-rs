// Generated macro for pack_size (function)
macro_rules! Depcratepack_size {
() => {
// Module: crate
// Provides: {"pack_size"}
// Dependencies: {}
# [inline] fn pack_size (n : u64) -> usize { (8 - ((n | 1) . leading_zeros () >> 3)) as usize }
};
}
