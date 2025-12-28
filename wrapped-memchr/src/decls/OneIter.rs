macro_rules! deps {
    () => {
        Iter!();
        One!();
    };
}

macro_rules! OneIter {
    () => {
        deps!();
        # [doc = " An iterator over all occurrences of a single byte in a haystack."] # [doc = ""] # [doc = " This iterator implements `DoubleEndedIterator`, which means it can also be"] # [doc = " used to find occurrences in reverse order."] # [doc = ""] # [doc = " This iterator is created by the [`One::iter`] method."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'a` refers to the lifetime of the underlying [`One`] searcher."] # [doc = " * `'h` refers to the lifetime of the haystack being searched."] # [derive (Clone , Debug)] pub struct OneIter < 'a , 'h > { searcher : & 'a One , it : generic :: Iter < 'h > , }
    };
}

OneIter!()