// Generated macro for Bytes (struct)
macro_rules! Depcrate_decode_bytesBytes {
() => {
// Module: crate::decode::bytes
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " A wrapper around `&[u8]` to read more efficiently."] # [doc = ""] # [doc = " This has a specialized implementation of `RmpWrite`"] # [doc = " and has error type [Infallible](core::convert::Infallible)."] # [doc = ""] # [doc = " This has the additional benefit of working on `#[no_std]` (unlike the builtin Read trait)"] # [doc = ""] # [doc = " See also [serde_bytes::Bytes](https://docs.rs/serde_bytes/0.11/serde_bytes/struct.Bytes.html)"] # [doc = ""] # [doc = " Unlike a plain `&[u8]` this also tracks an internal offset in the input (See [`Self::position`])."] # [doc = ""] # [doc = " This is used for (limited) compatibility with [`std::io::Cursor`]. Unlike a [Cursor](std::io::Cursor) it does"] # [doc = " not support mark/reset."] # [derive (Debug , Copy , Clone , Default , Eq , PartialEq , Hash , Ord , PartialOrd)] pub struct Bytes < 'a > { # [doc = " The internal position of the input buffer."] # [doc = ""] # [doc = " This is not required for correctness."] # [doc = " It is only used for error reporting (and to implement [`Self::position`])"] current_position : u64 , bytes : & 'a [u8] , }
};
}
