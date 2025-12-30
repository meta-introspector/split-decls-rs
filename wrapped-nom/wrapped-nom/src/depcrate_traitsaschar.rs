// Generated macro for AsChar (trait)
macro_rules! Depcrate_traitsAsChar {
() => {
// Module: crate::traits
// Provides: {"AsChar"}
// Dependencies: {}
# [doc = " Transforms common types to a char for basic token parsing"] # [allow (clippy :: len_without_is_empty)] pub trait AsChar : Copy { # [doc = " makes a char from self"] fn as_char (self) -> char ; # [doc = " Tests that self is an alphabetic character"] # [doc = ""] # [doc = " Warning: for `&str` it recognizes alphabetic"] # [doc = " characters outside of the 52 ASCII letters"] fn is_alpha (self) -> bool ; # [doc = " Tests that self is an alphabetic character"] # [doc = " or a decimal digit"] fn is_alphanum (self) -> bool ; # [doc = " Tests that self is a decimal digit"] fn is_dec_digit (self) -> bool ; # [doc = " Tests that self is an hex digit"] fn is_hex_digit (self) -> bool ; # [doc = " Tests that self is an octal digit"] fn is_oct_digit (self) -> bool ; # [doc = " Tests that self is a binary digit"] fn is_bin_digit (self) -> bool ; # [doc = " Gets the len in bytes for self"] fn len (self) -> usize ; # [doc = " Tests that self is ASCII space or tab"] fn is_space (self) -> bool ; # [doc = " Tests if byte is ASCII newline: \\n"] fn is_newline (self) -> bool ; }
};
}
