// Generated macro for Input (trait)
macro_rules! Depcrate_inputInput {
() => {
// Module: crate::input
// Provides: {"Input"}
// Dependencies: {}
# [doc = " An abstraction over input used in the matching engines."] pub trait Input { # [doc = " Return an encoding of the position at byte offset `i`."] fn at (& self , i : usize) -> InputAt ; # [doc = " Return the Unicode character occurring next to `at`."] # [doc = ""] # [doc = " If no such character could be decoded, then `Char` is absent."] fn next_char (& self , at : InputAt) -> Char ; # [doc = " Return the Unicode character occurring previous to `at`."] # [doc = ""] # [doc = " If no such character could be decoded, then `Char` is absent."] fn previous_char (& self , at : InputAt) -> Char ; # [doc = " Return true if the given empty width instruction matches at the"] # [doc = " input position given."] fn is_empty_match (& self , at : InputAt , empty : & InstEmptyLook) -> bool ; # [doc = " Scan the input for a matching prefix."] fn prefix_at (& self , prefixes : & LiteralSearcher , at : InputAt ,) -> Option < InputAt > ; # [doc = " The number of bytes in the input."] fn len (& self) -> usize ; # [doc = " Return the given input as a sequence of bytes."] fn as_bytes (& self) -> & [u8] ; }
};
}
