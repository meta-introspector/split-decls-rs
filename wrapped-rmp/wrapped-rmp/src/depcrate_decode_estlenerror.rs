// Generated macro for LenError (enum)
macro_rules! Depcrate_decode_estLenError {
() => {
// Module: crate::decode::est
// Provides: {"LenError"}
// Dependencies: {}
# [doc = " [`MessageLen`] result"] # [derive (Debug)] pub enum LenError { # [doc = " The message is truncated, and needs at least this many bytes to parse"] Truncated (NonZeroUsize) , # [doc = " The message is invalid or exceeded size limits"] ParseError , }
};
}
