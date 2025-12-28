macro_rules! deps {
    () => {
        Span!();
        Searcher!();
    };
}

macro_rules! FindIter {
    () => {
        deps!();
        # [doc = " An iterator over non-overlapping matches from a packed searcher."] # [doc = ""] # [doc = " The lifetime `'s` refers to the lifetime of the underlying [`Searcher`],"] # [doc = " while the lifetime `'h` refers to the lifetime of the haystack being"] # [doc = " searched."] # [derive (Debug)] pub struct FindIter < 's , 'h > { searcher : & 's Searcher , haystack : & 'h [u8] , span : Span , }
    };
}

FindIter!()