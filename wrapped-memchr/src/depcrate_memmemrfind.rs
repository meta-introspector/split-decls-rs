// Generated macro for rfind (function)
macro_rules! Depcrate_memmemrfind {
() => {
// Module: crate::memmem
// Provides: {"rfind"}
// Dependencies: {}
# [doc = " Returns the index of the last occurrence of the given needle."] # [doc = ""] # [doc = " Note that if you're are searching for the same needle in many different"] # [doc = " small haystacks, it may be faster to initialize a [`FinderRev`] once,"] # [doc = " and reuse it for each search."] # [doc = ""] # [doc = " # Complexity"] # [doc = ""] # [doc = " This routine is guaranteed to have worst case linear time complexity"] # [doc = " with respect to both the needle and the haystack. That is, this runs"] # [doc = " in `O(needle.len() + haystack.len())` time."] # [doc = ""] # [doc = " This routine is also guaranteed to have worst case constant space"] # [doc = " complexity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use memchr::memmem;"] # [doc = ""] # [doc = " let haystack = b\"foo bar baz\";"] # [doc = " assert_eq!(Some(0), memmem::rfind(haystack, b\"foo\"));"] # [doc = " assert_eq!(Some(4), memmem::rfind(haystack, b\"bar\"));"] # [doc = " assert_eq!(Some(8), memmem::rfind(haystack, b\"ba\"));"] # [doc = " assert_eq!(None, memmem::rfind(haystack, b\"quux\"));"] # [doc = " ```"] # [inline] pub fn rfind (haystack : & [u8] , needle : & [u8]) -> Option < usize > { if haystack . len () < 64 { rabinkarp :: FinderRev :: new (needle) . rfind (haystack , needle) } else { FinderRev :: new (needle) . rfind (haystack) } }
};
}
