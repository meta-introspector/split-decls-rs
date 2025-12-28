macro_rules! deps {
    () => {
        FinderRev!();
    };
}

macro_rules! FindRevIter {
    () => {
        deps!();
        # [doc = " An iterator over non-overlapping substring matches in reverse."] # [doc = ""] # [doc = " Matches are reported by the byte offset at which they begin."] # [doc = ""] # [doc = " `'h` is the lifetime of the haystack while `'n` is the lifetime of the"] # [doc = " needle."] # [derive (Clone , Debug)] pub struct FindRevIter < 'h , 'n > { haystack : & 'h [u8] , finder : FinderRev < 'n > , # [doc = " When searching with an empty needle, this gets set to `None` after"] # [doc = " we've yielded the last element at `0`."] pos : Option < usize > , }
    };
}

FindRevIter!()