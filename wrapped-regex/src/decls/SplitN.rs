macro_rules! SplitN {
    () => {
        # [doc = " An iterator over at most `N` substrings delimited by a regex match."] # [doc = ""] # [doc = " The last substring yielded by this iterator will be whatever remains after"] # [doc = " `N-1` splits."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'h` is the"] # [doc = " lifetime of the byte string being split."] # [doc = ""] # [doc = " This iterator is created by [`Regex::splitn`]."] # [doc = ""] # [doc = " # Time complexity"] # [doc = ""] # [doc = " Note that since an iterator runs potentially many searches on the haystack"] # [doc = " and since each search has worst case `O(m * n)` time complexity, the"] # [doc = " overall worst case time complexity for iteration is `O(m * n^2)`."] # [doc = ""] # [doc = " Although note that the worst case time here has an upper bound given"] # [doc = " by the `limit` parameter to [`Regex::splitn`]."] # [derive (Debug)] pub struct SplitN < 'r , 'h > { haystack : & 'h str , it : meta :: SplitN < 'r , 'h > , }
    };
}

SplitN!();