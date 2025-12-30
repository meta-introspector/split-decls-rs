// Generated macro for FindSubstring (trait)
macro_rules! Depcrate_traitsFindSubstring {
() => {
// Module: crate::traits
// Provides: {"FindSubstring"}
// Dependencies: {}
# [doc = " Look for a substring in self"] pub trait FindSubstring < T > { # [doc = " Returns the byte position of the substring if it is found"] fn find_substring (& self , substr : T) -> Option < usize > ; }
};
}
