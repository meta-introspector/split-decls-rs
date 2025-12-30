// Generated macro for read_unaligned_usize (function)
macro_rules! Depcrate_byteset_scalarread_unaligned_usize {
() => {
// Module: crate::byteset::scalar
// Provides: {"read_unaligned_usize"}
// Dependencies: {}
unsafe fn read_unaligned_usize (ptr : * const u8) -> usize { (ptr as * const usize) . read_unaligned () }
};
}
