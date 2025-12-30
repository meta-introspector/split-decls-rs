// Generated macro for reverse_bits (function)
macro_rules! Depcrate_inflate_corereverse_bits {
() => {
// Module: crate::inflate::core
// Provides: {"reverse_bits"}
// Dependencies: {}
# [cfg (all (not (any (feature = "rustc-dep-of-std" , target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "loongarch64")) , feature = "with-alloc"))] fn reverse_bits (n : u16) -> u16 { static REVERSED_BITS_LOOKUP : [u16 ; 512] = { let mut table = [0 ; 512] ; let mut i = 0 ; while i < 512 { table [i] = (i as u16) . reverse_bits () ; i += 1 ; } table } ; REVERSED_BITS_LOOKUP [n as usize] }
};
}
