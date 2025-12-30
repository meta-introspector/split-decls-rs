// Generated macro for CharIndices (struct)
macro_rules! Depcrate_utf8CharIndices {
() => {
// Module: crate::utf8
// Provides: {"CharIndices"}
// Dependencies: {}
# [doc = " An iterator over Unicode scalar values in a byte string and their"] # [doc = " byte index positions."] # [doc = ""] # [doc = " When invalid UTF-8 byte sequences are found, they are substituted with the"] # [doc = " Unicode replacement codepoint (`U+FFFD`) using the"] # [doc = " [\"maximal subpart\" strategy](https://www.unicode.org/review/pr-121.html)."] # [doc = ""] # [doc = " Note that this is slightly different from the `CharIndices` iterator"] # [doc = " provided by the standard library. Aside from working on possibly invalid"] # [doc = " UTF-8, this iterator provides both the corresponding starting and ending"] # [doc = " byte indices of each codepoint yielded. The ending position is necessary to"] # [doc = " slice the original byte string when invalid UTF-8 bytes are converted into"] # [doc = " a Unicode replacement codepoint, since a single replacement codepoint can"] # [doc = " substitute anywhere from 1 to 3 invalid bytes (inclusive)."] # [doc = ""] # [doc = " This iterator is created by the"] # [doc = " [`char_indices`](trait.ByteSlice.html#method.char_indices) method provided"] # [doc = " by the [`ByteSlice`](trait.ByteSlice.html) extension trait for `&[u8]`."] # [derive (Clone , Debug)] pub struct CharIndices < 'a > { bs : & 'a [u8] , forward_index : usize , reverse_index : usize , }
};
}
