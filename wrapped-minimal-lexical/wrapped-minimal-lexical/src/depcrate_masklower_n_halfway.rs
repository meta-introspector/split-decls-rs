// Generated macro for lower_n_halfway (function)
macro_rules! Depcrate_masklower_n_halfway {
() => {
// Module: crate::mask
// Provides: {"lower_n_halfway"}
// Dependencies: {}
# [doc = " Calculate the halfway point for the lower `n` bits."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use minimal_lexical::mask::lower_n_halfway;"] # [doc = " # pub fn main() {"] # [doc = " assert_eq!(lower_n_halfway(2), 0b10);"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn lower_n_halfway (n : u64) -> u64 { debug_assert ! (n <= 64 , "lower_n_halfway() overflow in shl.") ; match n == 0 { true => 0 , false => nth_bit (n - 1) , } }
};
}
