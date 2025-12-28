macro_rules! Chars {
    () => {
        # [doc = " An iterator over Unicode scalar values in a byte string."] # [doc = ""] # [doc = " When invalid UTF-8 byte sequences are found, they are substituted with the"] # [doc = " Unicode replacement codepoint (`U+FFFD`) using the"] # [doc = " [\"maximal subpart\" strategy](https://www.unicode.org/review/pr-121.html)."] # [doc = ""] # [doc = " This iterator is created by the"] # [doc = " [`chars`](trait.ByteSlice.html#method.chars) method provided by the"] # [doc = " [`ByteSlice`](trait.ByteSlice.html) extension trait for `&[u8]`."] # [derive (Clone , Debug)] pub struct Chars < 'a > { bs : & 'a [u8] , }
    };
}

Chars!();