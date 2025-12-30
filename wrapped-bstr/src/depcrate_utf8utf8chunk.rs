// Generated macro for Utf8Chunk (struct)
macro_rules! Depcrate_utf8Utf8Chunk {
() => {
// Module: crate::utf8
// Provides: {"Utf8Chunk"}
// Dependencies: {}
# [doc = " A chunk of valid UTF-8, possibly followed by invalid UTF-8 bytes."] # [doc = ""] # [doc = " This is yielded by the"] # [doc = " [`Utf8Chunks`](struct.Utf8Chunks.html)"] # [doc = " iterator, which can be created via the"] # [doc = " [`ByteSlice::utf8_chunks`](trait.ByteSlice.html#method.utf8_chunks)"] # [doc = " method."] # [doc = ""] # [doc = " The `'a` lifetime parameter corresponds to the lifetime of the bytes that"] # [doc = " are being iterated over."] # [cfg_attr (test , derive (Debug , PartialEq))] pub struct Utf8Chunk < 'a > { # [doc = " A valid UTF-8 piece, at the start, end, or between invalid UTF-8 bytes."] # [doc = ""] # [doc = " This is empty between adjacent invalid UTF-8 byte sequences."] valid : & 'a str , # [doc = " A sequence of invalid UTF-8 bytes."] # [doc = ""] # [doc = " Can only be empty in the last chunk."] # [doc = ""] # [doc = " Should be replaced by a single unicode replacement character, if not"] # [doc = " empty."] invalid : & 'a BStr , # [doc = " Indicates whether the invalid sequence could've been valid if there"] # [doc = " were more bytes."] # [doc = ""] # [doc = " Can only be true in the last chunk."] incomplete : bool , }
};
}
