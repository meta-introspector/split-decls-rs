macro_rules! memrchr {
    () => {
        # [doc = " Search for the last occurrence of a byte in a slice."] # [doc = ""] # [doc = " This returns the index corresponding to the last occurrence of `needle` in"] # [doc = " `haystack`, or `None` if one is not found. If an index is returned, it is"] # [doc = " guaranteed to be less than `haystack.len()`."] # [doc = ""] # [doc = " While this is semantically the same as something like"] # [doc = " `haystack.iter().rposition(|&b| b == needle)`, this routine will attempt to"] # [doc = " use highly optimized vector operations that can be an order of magnitude"] # [doc = " faster (or more)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to find the last position of a byte in a byte string."] # [doc = ""] # [doc = " ```"] # [doc = " use memchr::memrchr;"] # [doc = ""] # [doc = " let haystack = b\"the quick brown fox\";"] # [doc = " assert_eq!(memrchr(b'o', haystack), Some(17));"] # [doc = " ```"] # [inline] pub fn memrchr (needle : u8 , haystack : & [u8]) -> Option < usize > { unsafe { generic :: search_slice_with_raw (haystack , | start , end | { memrchr_raw (needle , start , end) }) } }
    };
}

memrchr!()