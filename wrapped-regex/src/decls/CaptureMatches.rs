macro_rules! deps {
    () => {
        Captures!();
    };
}

macro_rules! CaptureMatches {
    () => {
        deps!();
        # [doc = " An iterator over all non-overlapping capture matches in a haystack."] # [doc = ""] # [doc = " This iterator yields [`Captures`] values. The iterator stops when no more"] # [doc = " matches can be found."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'h` is the"] # [doc = " lifetime of the matched string."] # [doc = ""] # [doc = " This iterator is created by [`Regex::captures_iter`]."] # [doc = ""] # [doc = " # Time complexity"] # [doc = ""] # [doc = " Note that since an iterator runs potentially many searches on the haystack"] # [doc = " and since each search has worst case `O(m * n)` time complexity, the"] # [doc = " overall worst case time complexity for iteration is `O(m * n^2)`."] # [derive (Debug)] pub struct CaptureMatches < 'r , 'h > { haystack : & 'h str , it : meta :: CapturesMatches < 'r , 'h > , }
    };
}

CaptureMatches!();