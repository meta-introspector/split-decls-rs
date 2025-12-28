macro_rules! FindReverse {
    () => {
        # [doc = " An iterator over non-overlapping substring matches in reverse."] # [doc = ""] # [doc = " Matches are reported by the byte offset at which they begin."] # [doc = ""] # [doc = " `'h` is the lifetime of the haystack while `'n` is the lifetime of the"] # [doc = " needle."] # [derive (Clone , Debug)] pub struct FindReverse < 'h , 'n > { it : memmem :: FindRevIter < 'h , 'n > , haystack : & 'h [u8] , needle : & 'n [u8] , }
    };
}

FindReverse!();