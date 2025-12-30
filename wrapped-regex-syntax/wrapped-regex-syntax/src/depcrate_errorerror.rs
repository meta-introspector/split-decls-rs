// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " This error type encompasses any error that can be returned by this crate."] # [doc = ""] # [doc = " This error type is marked as `non_exhaustive`. This means that adding a"] # [doc = " new variant is not considered a breaking change."] # [non_exhaustive] # [derive (Clone , Debug , Eq , PartialEq)] pub enum Error { # [doc = " An error that occurred while translating concrete syntax into abstract"] # [doc = " syntax (AST)."] Parse (ast :: Error) , # [doc = " An error that occurred while translating abstract syntax into a high"] # [doc = " level intermediate representation (HIR)."] Translate (hir :: Error) , }
};
}
