// Generated macro for CChar (enum)
macro_rules! Depcrate_literalCChar {
() => {
// Module: crate::literal
// Provides: {"CChar"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] # [doc = " Representation of a C character"] pub enum CChar { # [doc = " A character that can be represented as a `char`"] Char (char) , # [doc = " Any other character (8-bit characters, unicode surrogates, etc.)"] Raw (u64) , }
};
}
