macro_rules! deps {
    () => {
        Finder!();
        FindIter!();
    };
}

macro_rules! find_iter {
    () => {
        deps!();
        # [doc = " Returns an iterator over all non-overlapping occurrences of a substring in"] # [doc = " a haystack."] # [doc = ""] # [doc = " # Complexity"] # [doc = ""] # [doc = " This routine is guaranteed to have worst case linear time complexity"] # [doc = " with respect to both the needle and the haystack. That is, this runs"] # [doc = " in `O(needle.len() + haystack.len())` time."] # [doc = ""] # [doc = " This routine is also guaranteed to have worst case constant space"] # [doc = " complexity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use memchr::memmem;"] # [doc = ""] # [doc = " let haystack = b\"foo bar foo baz foo\";"] # [doc = " let mut it = memmem::find_iter(haystack, b\"foo\");"] # [doc = " assert_eq!(Some(0), it.next());"] # [doc = " assert_eq!(Some(8), it.next());"] # [doc = " assert_eq!(Some(16), it.next());"] # [doc = " assert_eq!(None, it.next());"] # [doc = " ```"] # [inline] pub fn find_iter < 'h , 'n , N : 'n + ? Sized + AsRef < [u8] > > (haystack : & 'h [u8] , needle : & 'n N ,) -> FindIter < 'h , 'n > { FindIter :: new (haystack , Finder :: new (needle)) }
    };
}

find_iter!()