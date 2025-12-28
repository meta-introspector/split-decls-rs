macro_rules! deps {
    () => {
        Finder!();
    };
}

macro_rules! find {
    () => {
        deps!();
        # [doc = " Returns the index of the first occurrence of the given needle."] # [doc = ""] # [doc = " Note that if you're are searching for the same needle in many different"] # [doc = " small haystacks, it may be faster to initialize a [`Finder`] once,"] # [doc = " and reuse it for each search."] # [doc = ""] # [doc = " # Complexity"] # [doc = ""] # [doc = " This routine is guaranteed to have worst case linear time complexity"] # [doc = " with respect to both the needle and the haystack. That is, this runs"] # [doc = " in `O(needle.len() + haystack.len())` time."] # [doc = ""] # [doc = " This routine is also guaranteed to have worst case constant space"] # [doc = " complexity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use memchr::memmem;"] # [doc = ""] # [doc = " let haystack = b\"foo bar baz\";"] # [doc = " assert_eq!(Some(0), memmem::find(haystack, b\"foo\"));"] # [doc = " assert_eq!(Some(4), memmem::find(haystack, b\"bar\"));"] # [doc = " assert_eq!(None, memmem::find(haystack, b\"quux\"));"] # [doc = " ```"] # [inline] pub fn find (haystack : & [u8] , needle : & [u8]) -> Option < usize > { if haystack . len () < 64 { rabinkarp :: Finder :: new (needle) . find (haystack , needle) } else { Finder :: new (needle) . find (haystack) } }
    };
}

find!();