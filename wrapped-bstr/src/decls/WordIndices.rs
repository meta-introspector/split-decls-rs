macro_rules! deps {
    () => {
        WordsWithBreakIndices!();
    };
}

macro_rules! WordIndices {
    () => {
        deps!();
        # [doc = " An iterator over words in a byte string and their byte index positions."] # [doc = ""] # [doc = " This iterator is typically constructed by"] # [doc = " [`ByteSlice::word_indices`](trait.ByteSlice.html#method.word_indices)."] # [doc = ""] # [doc = " This is similar to the"] # [doc = " [`WordsWithBreakIndices`](struct.WordsWithBreakIndices.html) iterator,"] # [doc = " except it only returns elements that contain a \"word\" character. A"] # [doc = " word character is defined by UTS #18 (Annex C) to be the combination"] # [doc = " of the `Alphabetic` and `Join_Control` properties, along with the"] # [doc = " `Decimal_Number`, `Mark` and `Connector_Punctuation` general categories."] # [doc = ""] # [doc = " Since words are made up of one or more codepoints, this iterator"] # [doc = " yields `&str` elements (along with their start and end byte offsets)."] # [doc = " When invalid UTF-8 is encountered, replacement codepoints are"] # [doc = " [substituted](index.html#handling-of-invalid-utf-8). Because of this, the"] # [doc = " indices yielded by this iterator may not correspond to the length of the"] # [doc = " word yielded with those indices. For example, when this iterator encounters"] # [doc = " `\\xFF` in the byte string, then it will yield a pair of indices ranging"] # [doc = " over a single byte, but will provide an `&str` equivalent to `\"\\u{FFFD}\"`,"] # [doc = " which is three bytes in length. However, when given only valid UTF-8, then"] # [doc = " all indices are in exact correspondence with their paired word."] # [doc = ""] # [doc = " This iterator yields words in accordance with the default word boundary"] # [doc = " rules specified in"] # [doc = " [UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Word_Boundaries)."] # [doc = " In particular, this may not be suitable for Japanese and Chinese scripts"] # [doc = " that do not use spaces between words."] # [derive (Clone , Debug)] pub struct WordIndices < 'a > (WordsWithBreakIndices < 'a >) ;
    };
}

WordIndices!();