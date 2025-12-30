// Generated macro for Copies (struct)
macro_rules! Depcrate_rewritesCopies {
() => {
// Module: crate::rewrites
// Provides: {"Copies"}
// Dependencies: {}
# [doc = " Under which circumstances we consider a file to be a copy."] # [derive (Debug , Copy , Clone , PartialEq)] pub struct Copies { # [doc = " The set of files to search when finding the source of copies."] pub source : CopySource , # [doc = " Equivalent to [`Rewrites::percentage`], but used for copy tracking."] # [doc = ""] # [doc = " Useful to have similarity-based rename tracking and cheaper copy tracking."] pub percentage : Option < f32 > , }
};
}
