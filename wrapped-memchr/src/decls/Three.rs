macro_rules! Three {
    () => {
        # [doc = " Finds all occurrences of three bytes in a haystack."] # [doc = ""] # [doc = " That is, this reports matches of one of three possible bytes. For example,"] # [doc = " searching for `a`, `b` or `o` in `afoobar` would report matches at offsets"] # [doc = " `0`, `2`, `3`, `4` and `5`."] # [derive (Clone , Copy , Debug)] pub struct Three (generic :: Three < __m128i >) ;
    };
}

Three!();