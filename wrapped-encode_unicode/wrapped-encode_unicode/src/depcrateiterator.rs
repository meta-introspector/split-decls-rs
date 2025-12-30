// Generated macro for iterator (module)
macro_rules! Depcrateiterator {
() => {
// Module: crate
// Provides: {"iterator"}
// Dependencies: {}
pub mod iterator { # ! [doc = " Iterator types that you should rarely need to name"] pub use crate :: utf8_iterators :: { Utf8Iterator , Utf8CharSplitter , Utf8Chars , Utf8CharIndices } ; pub use crate :: utf16_iterators :: { Utf16Iterator , Utf16CharSplitter , Utf16Chars , Utf16CharIndices } ; pub use crate :: decoding_iterators :: { Utf8CharMerger , Utf8CharDecoder } ; pub use crate :: decoding_iterators :: { Utf16CharMerger , Utf16CharDecoder } ; }
};
}
