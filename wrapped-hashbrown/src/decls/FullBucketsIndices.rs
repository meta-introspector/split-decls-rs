macro_rules! deps {
    () => {
        BitMaskIter!();
    };
}

macro_rules! FullBucketsIndices {
    () => {
        deps!();
        # [doc = " Iterator which returns an index of every full bucket in the table."] # [doc = ""] # [doc = " For maximum flexibility this iterator is not bound by a lifetime, but you"] # [doc = " must observe several rules when using it:"] # [doc = " - You must not free the hash table while iterating (including via growing/shrinking)."] # [doc = " - It is fine to erase a bucket that has been yielded by the iterator."] # [doc = " - Erasing a bucket that has not yet been yielded by the iterator may still"] # [doc = "   result in the iterator yielding index of that bucket."] # [doc = " - It is unspecified whether an element inserted after the iterator was"] # [doc = "   created will be yielded by that iterator."] # [doc = " - The order in which the iterator yields indices of the buckets is unspecified"] # [doc = "   and may change in the future."] pub (crate) struct FullBucketsIndices { current_group : BitMaskIter , group_first_index : usize , ctrl : NonNull < u8 > , items : usize , }
    };
}

FullBucketsIndices!();