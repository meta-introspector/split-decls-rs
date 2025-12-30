// Generated macro for Needle (trait)
macro_rules! Depcrate_needleNeedle {
() => {
// Module: crate::needle
// Provides: {"Needle"}
// Dependencies: {}
# [doc = " Needle an interface for search of a match in a buffer."] pub trait Needle { # [doc = " Function returns all matches that were occured."] fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > ; }
};
}
