macro_rules! deps {
    () => {
        TaggedLen!();
        SmallVec!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " An iterator that consumes a `SmallVec` and yields its items by value."] # [doc = ""] # [doc = " Returned from [`SmallVec::into_iter`][1]."] # [doc = ""] # [doc = " [1]: struct.SmallVec.html#method.into_iter"] pub struct IntoIter < T , const N : usize > { raw : RawSmallVec < T , N > , begin : usize , end : TaggedLen , _marker : PhantomData < T > , }
    };
}

IntoIter!()