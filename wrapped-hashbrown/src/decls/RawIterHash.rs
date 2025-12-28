macro_rules! deps {
    () => {
        RawIterHashInner!();
        RawTable!();
    };
}

macro_rules! RawIterHash {
    () => {
        deps!();
        # [doc = " Iterator over occupied buckets that could match a given hash."] # [doc = ""] # [doc = " `RawTable` only stores 7 bits of the hash value, so this iterator may return"] # [doc = " items that have a hash value different than the one provided. You should"] # [doc = " always validate the returned values before using them."] # [doc = ""] # [doc = " For maximum flexibility this iterator is not bound by a lifetime, but you"] # [doc = " must observe several rules when using it:"] # [doc = " - You must not free the hash table while iterating (including via growing/shrinking)."] # [doc = " - It is fine to erase a bucket that has been yielded by the iterator."] # [doc = " - Erasing a bucket that has not yet been yielded by the iterator may still"] # [doc = "   result in the iterator yielding that bucket."] # [doc = " - It is unspecified whether an element inserted after the iterator was"] # [doc = "   created will be yielded by that iterator."] # [doc = " - The order in which the iterator yields buckets is unspecified and may"] # [doc = "   change in the future."] pub struct RawIterHash < T > { inner : RawIterHashInner , _marker : PhantomData < T > , }
    };
}

RawIterHash!();