// Generated macro for Error (struct)
macro_rules! Depcrate_hirError {
() => {
// Module: crate::hir
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that can occur while translating an `Ast` to a `Hir`."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Error { # [doc = " The kind of error."] kind : ErrorKind , # [doc = " The original pattern that the translator's Ast was parsed from. Every"] # [doc = " span in an error is a valid range into this string."] pattern : String , # [doc = " The span of this error, derived from the Ast given to the translator."] span : Span , }
};
}
