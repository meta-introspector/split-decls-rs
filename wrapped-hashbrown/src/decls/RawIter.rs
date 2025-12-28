macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! RawIter {
    () => {
        deps!();
        # [doc = " Iterator which returns a raw pointer to every full bucket in the table."] # [doc = ""] # [doc = " For maximum flexibility this iterator is not bound by a lifetime, but you"] # [doc = " must observe several rules when using it:"] # [doc = " - You must not free the hash table while iterating (including via growing/shrinking)."] # [doc = " - It is fine to erase a bucket that has been yielded by the iterator."] # [doc = " - Erasing a bucket that has not yet been yielded by the iterator may still"] # [doc = "   result in the iterator yielding that bucket (unless `reflect_remove` is called)."] # [doc = " - It is unspecified whether an element inserted after the iterator was"] # [doc = "   created will be yielded by that iterator (unless `reflect_insert` is called)."] # [doc = " - The order in which the iterator yields bucket is unspecified and may"] # [doc = "   change in the future."] pub struct RawIter < T > { pub (crate) iter : RawIterRange < T > , items : usize , }
    };
}

RawIter!();