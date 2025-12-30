// Generated macro for lower_n_mask (function)
macro_rules! Depcrate_masklower_n_mask {
() => {
// Module: crate::mask
// Provides: {"lower_n_mask"}
// Dependencies: {}
# [doc = " Generate a bitwise mask for the lower `n` bits."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use minimal_lexical::mask::lower_n_mask;"] # [doc = " # pub fn main() {"] # [doc = " assert_eq!(lower_n_mask(2), 0b11);"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn lower_n_mask (n : u64) -> u64 { debug_assert ! (n <= 64 , "lower_n_mask() overflow in shl.") ; match n == 64 { true => 0xffff_ffff_ffff_ffff , false => (1 << n) - 1 , } }
};
}
