macro_rules! Two {
    () => {
        # [doc = " Finds all occurrences of two bytes in a haystack."] # [doc = ""] # [doc = " That is, this reports matches of one of two possible bytes. For example,"] # [doc = " searching for `a` or `b` in `afoobar` would report matches at offsets `0`,"] # [doc = " `4` and `5`."] # [derive (Clone , Copy , Debug)] pub struct Two (generic :: Two < __m128i >) ;
    };
}

Two!()