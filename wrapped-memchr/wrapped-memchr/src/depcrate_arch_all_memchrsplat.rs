// Generated macro for splat (function)
macro_rules! Depcrate_arch_all_memchrsplat {
() => {
// Module: crate::arch::all::memchr
// Provides: {"splat"}
// Dependencies: {}
# [doc = " Repeat the given byte into a word size number. That is, every 8 bits"] # [doc = " is equivalent to the given byte. For example, if `b` is `\\x4E` or"] # [doc = " `01001110` in binary, then the returned value on a 32-bit system would be:"] # [doc = " `01001110_01001110_01001110_01001110`."] # [inline (always)] const fn splat (b : u8) -> usize { (b as usize) * (usize :: MAX / 255) }
};
}
