// Generated macro for read_usize_unaligned (function)
macro_rules! Depcrate_mem_implsread_usize_unaligned {
() => {
// Module: crate::mem::impls
// Provides: {"read_usize_unaligned"}
// Dependencies: {}
# [cfg (feature = "mem-unaligned")] unsafe fn read_usize_unaligned (x : * const usize) -> usize { let x_read = (x as * const [u8 ; core :: mem :: size_of :: < usize > ()]) . read () ; usize :: from_ne_bytes (x_read) }
};
}
