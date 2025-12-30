// Generated macro for Split (struct)
macro_rules! Depcrate_stringSplit {
() => {
// Module: crate::string
// Provides: {"Split"}
// Dependencies: {}
# [doc = " An iterator over all substrings delimited by a regex match."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'h` is the"] # [doc = " lifetime of the byte string being split."] # [doc = ""] # [doc = " This iterator is created by [`Regex::split`]."] # [doc = ""] # [doc = " # Time complexity"] # [doc = ""] # [doc = " Note that since an iterator runs potentially many searches on the haystack"] # [doc = " and since each search has worst case `O(m * n)` time complexity, the"] # [doc = " overall worst case time complexity for iteration is `O(m * n^2)`."] # [derive (Debug)] pub struct Split < 'r , 'h > { haystack : & 'h str , finder : Matches < 'r , 'h > , last : usize , }
};
}
