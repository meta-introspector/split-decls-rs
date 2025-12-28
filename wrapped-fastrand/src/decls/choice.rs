macro_rules! choice {
    () => {
        # [doc = " Choose an item from an iterator at random."] # [doc = ""] # [doc = " This function may have an unexpected result if the `len()` property of the"] # [doc = " iterator does not match the actual number of items in the iterator. If"] # [doc = " the iterator is empty, this returns `None`."] # [inline] pub fn choice < I > (iter : I) -> Option < I :: Item > where I : IntoIterator , I :: IntoIter : ExactSizeIterator , { with_rng (| r | r . choice (iter)) }
    };
}

choice!();