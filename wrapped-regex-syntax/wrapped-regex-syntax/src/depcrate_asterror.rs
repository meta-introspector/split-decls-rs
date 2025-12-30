// Generated macro for Error (struct)
macro_rules! Depcrate_astError {
() => {
// Module: crate::ast
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that occurred while parsing a regular expression into an abstract"] # [doc = " syntax tree."] # [doc = ""] # [doc = " Note that not all ASTs represents a valid regular expression. For example,"] # [doc = " an AST is constructed without error for `\\p{Quux}`, but `Quux` is not a"] # [doc = " valid Unicode property name. That particular error is reported when"] # [doc = " translating an AST to the high-level intermediate representation (`HIR`)."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Error { # [doc = " The kind of error."] kind : ErrorKind , # [doc = " The original pattern that the parser generated the error from. Every"] # [doc = " span in an error is a valid range into this string."] pattern : String , # [doc = " The span of this error."] span : Span , }
};
}
