// Generated macro for Error (enum)
macro_rules! Depcrate_de_errorError {
() => {
// Module: crate::de::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error occurred during deserialization"] # [derive (Clone , Debug)] pub enum Error < T > { # [doc = " An error occurred while reading bytes"] # [doc = ""] # [doc = " Contains the underlying error returned while reading."] Io (T) , # [doc = " An error occurred while parsing bytes"] # [doc = ""] # [doc = " Contains the offset into the stream where the syntax error occurred."] Syntax (usize) , # [doc = " An error occurred while processing a parsed value"] # [doc = ""] # [doc = " Contains a description of the error that occurred and (optionally)"] # [doc = " the offset into the stream indicating the start of the item being"] # [doc = " processed when the error occurred."] Semantic (Option < usize > , String) , # [doc = " The input caused serde to recurse too much"] # [doc = ""] # [doc = " This error prevents a stack overflow."] RecursionLimitExceeded , }
};
}
