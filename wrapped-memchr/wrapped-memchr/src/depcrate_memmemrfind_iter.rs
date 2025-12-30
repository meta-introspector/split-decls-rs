// Generated macro for rfind_iter (function)
macro_rules! Depcrate_memmemrfind_iter {
() => {
// Module: crate::memmem
// Provides: {"rfind_iter"}
// Dependencies: {}
# [doc = " Returns a reverse iterator over all non-overlapping occurrences of a"] # [doc = " substring in a haystack."] # [doc = ""] # [doc = " # Complexity"] # [doc = ""] # [doc = " This routine is guaranteed to have worst case linear time complexity"] # [doc = " with respect to both the needle and the haystack. That is, this runs"] # [doc = " in `O(needle.len() + haystack.len())` time."] # [doc = ""] # [doc = " This routine is also guaranteed to have worst case constant space"] # [doc = " complexity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use memchr::memmem;"] # [doc = ""] # [doc = " let haystack = b\"foo bar foo baz foo\";"] # [doc = " let mut it = memmem::rfind_iter(haystack, b\"foo\");"] # [doc = " assert_eq!(Some(16), it.next());"] # [doc = " assert_eq!(Some(8), it.next());"] # [doc = " assert_eq!(Some(0), it.next());"] # [doc = " assert_eq!(None, it.next());"] # [doc = " ```"] # [inline] pub fn rfind_iter < 'h , 'n , N : 'n + ? Sized + AsRef < [u8] > > (haystack : & 'h [u8] , needle : & 'n N ,) -> FindRevIter < 'h , 'n > { FindRevIter :: new (haystack , FinderRev :: new (needle)) }
};
}
