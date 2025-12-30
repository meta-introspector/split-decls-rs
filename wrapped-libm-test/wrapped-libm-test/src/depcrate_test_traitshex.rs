// Generated macro for Hex (trait)
macro_rules! Depcrate_test_traitsHex {
() => {
// Module: crate::test_traits
// Provides: {"Hex"}
// Dependencies: {}
# [doc = " A helper trait to print something as hex with the correct number of nibbles, e.g. a `u32`"] # [doc = " will always print with `0x` followed by 8 digits."] # [doc = ""] # [doc = " This is only used for printing errors so allocating is okay."] pub trait Hex : Copy { # [doc = " Hex integer syntax."] fn hex (self) -> String ; # [doc = " Hex float syntax."] fn hexf (self) -> String ; }
};
}
