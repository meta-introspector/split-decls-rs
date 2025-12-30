// Generated macro for SentenceIndices (struct)
macro_rules! Depcrate_unicode_sentenceSentenceIndices {
() => {
// Module: crate::unicode::sentence
// Provides: {"SentenceIndices"}
// Dependencies: {}
# [doc = " An iterator over sentences in a byte string, along with their byte offsets."] # [doc = ""] # [doc = " This iterator is typically constructed by"] # [doc = " [`ByteSlice::sentence_indices`](trait.ByteSlice.html#method.sentence_indices)."] # [doc = ""] # [doc = " Sentences typically include their trailing punctuation and whitespace."] # [doc = ""] # [doc = " Since sentences are made up of one or more codepoints, this iterator"] # [doc = " yields `&str` elements (along with their start and end byte offsets)."] # [doc = " When invalid UTF-8 is encountered, replacement codepoints are"] # [doc = " [substituted](index.html#handling-of-invalid-utf-8). Because of this, the"] # [doc = " indices yielded by this iterator may not correspond to the length of the"] # [doc = " sentence yielded with those indices. For example, when this iterator"] # [doc = " encounters `\\xFF` in the byte string, then it will yield a pair of indices"] # [doc = " ranging over a single byte, but will provide an `&str` equivalent to"] # [doc = " `\"\\u{FFFD}\"`, which is three bytes in length. However, when given only"] # [doc = " valid UTF-8, then all indices are in exact correspondence with their paired"] # [doc = " word."] # [doc = ""] # [doc = " This iterator yields words in accordance with the default sentence boundary"] # [doc = " rules specified in"] # [doc = " [UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Sentence_Boundaries)."] # [derive (Clone , Debug)] pub struct SentenceIndices < 'a > { bs : & 'a [u8] , forward_index : usize , }
};
}
