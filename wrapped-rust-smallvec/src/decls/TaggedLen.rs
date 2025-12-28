macro_rules! TaggedLen {
    () => {
        # [doc = " Vec guarantees that its length is always less than [`isize::MAX`] in *bytes*."] # [doc = ""] # [doc = " For a non ZST, this means that the length is less than `isize::MAX` objects, which implies we"] # [doc = " have at least one free bit we can use. We use the least significant bit for the tag. And store"] # [doc = " the length in the `usize::BITS - 1` most significant bits."] # [doc = ""] # [doc = " For a ZST, we never use the heap, so we just store the length directly."] # [repr (transparent)] # [derive (Clone , Copy)] struct TaggedLen (usize) ;
    };
}

TaggedLen!()