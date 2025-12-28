macro_rules! Sentences {
    () => {
        # [doc = " An iterator over sentences in a byte string."] # [doc = ""] # [doc = " This iterator is typically constructed by"] # [doc = " [`ByteSlice::sentences`](trait.ByteSlice.html#method.sentences)."] # [doc = ""] # [doc = " Sentences typically include their trailing punctuation and whitespace."] # [doc = ""] # [doc = " Since sentences are made up of one or more codepoints, this iterator yields"] # [doc = " `&str` elements. When invalid UTF-8 is encountered, replacement codepoints"] # [doc = " are [substituted](index.html#handling-of-invalid-utf-8)."] # [doc = ""] # [doc = " This iterator yields words in accordance with the default sentence boundary"] # [doc = " rules specified in"] # [doc = " [UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Sentence_Boundaries)."] # [derive (Clone , Debug)] pub struct Sentences < 'a > { bs : & 'a [u8] , }
    };
}

Sentences!()