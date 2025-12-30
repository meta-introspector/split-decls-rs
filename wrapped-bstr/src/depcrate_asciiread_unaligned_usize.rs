// Generated macro for read_unaligned_usize (function)
macro_rules! Depcrate_asciiread_unaligned_usize {
() => {
// Module: crate::ascii
// Provides: {"read_unaligned_usize"}
// Dependencies: {}
# [cfg (any (test , miri , not (target_arch = "x86_64")))] unsafe fn read_unaligned_usize (ptr : * const u8) -> usize { use core :: ptr ; let mut n : usize = 0 ; ptr :: copy_nonoverlapping (ptr , & mut n as * mut _ as * mut u8 , USIZE_BYTES) ; n }
};
}
