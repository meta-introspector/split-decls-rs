// Generated macro for Words (struct)
macro_rules! Depcrate_unicode_wordWords {
() => {
// Module: crate::unicode::word
// Provides: {"Words"}
// Dependencies: {}
# [doc = " An iterator over words in a byte string."] # [doc = ""] # [doc = " This iterator is typically constructed by"] # [doc = " [`ByteSlice::words`](trait.ByteSlice.html#method.words)."] # [doc = ""] # [doc = " This is similar to the [`WordsWithBreaks`](struct.WordsWithBreaks.html)"] # [doc = " iterator, except it only returns elements that contain a \"word\" character."] # [doc = " A word character is defined by UTS #18 (Annex C) to be the combination"] # [doc = " of the `Alphabetic` and `Join_Control` properties, along with the"] # [doc = " `Decimal_Number`, `Mark` and `Connector_Punctuation` general categories."] # [doc = ""] # [doc = " Since words are made up of one or more codepoints, this iterator yields"] # [doc = " `&str` elements. When invalid UTF-8 is encountered, replacement codepoints"] # [doc = " are [substituted](index.html#handling-of-invalid-utf-8)."] # [doc = ""] # [doc = " This iterator yields words in accordance with the default word boundary"] # [doc = " rules specified in"] # [doc = " [UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Word_Boundaries)."] # [doc = " In particular, this may not be suitable for Japanese and Chinese scripts"] # [doc = " that do not use spaces between words."] # [derive (Clone , Debug)] pub struct Words < 'a > (WordsWithBreaks < 'a >) ;
};
}
