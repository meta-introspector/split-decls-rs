// Generated macro for WordsWithBreaks (struct)
macro_rules! Depcrate_unicode_wordWordsWithBreaks {
() => {
// Module: crate::unicode::word
// Provides: {"WordsWithBreaks"}
// Dependencies: {}
# [doc = " An iterator over all word breaks in a byte string."] # [doc = ""] # [doc = " This iterator is typically constructed by"] # [doc = " [`ByteSlice::words_with_breaks`](trait.ByteSlice.html#method.words_with_breaks)."] # [doc = ""] # [doc = " This iterator yields not only all words, but the content that comes between"] # [doc = " words. In particular, if all elements yielded by this iterator are"] # [doc = " concatenated, then the result is the original string (subject to Unicode"] # [doc = " replacement codepoint substitutions)."] # [doc = ""] # [doc = " Since words are made up of one or more codepoints, this iterator yields"] # [doc = " `&str` elements. When invalid UTF-8 is encountered, replacement codepoints"] # [doc = " are [substituted](index.html#handling-of-invalid-utf-8)."] # [doc = ""] # [doc = " This iterator yields words in accordance with the default word boundary"] # [doc = " rules specified in"] # [doc = " [UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Word_Boundaries)."] # [doc = " In particular, this may not be suitable for Japanese and Chinese scripts"] # [doc = " that do not use spaces between words."] # [derive (Clone , Debug)] pub struct WordsWithBreaks < 'a > { bs : & 'a [u8] , }
};
}
