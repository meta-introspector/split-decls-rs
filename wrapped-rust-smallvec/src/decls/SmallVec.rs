macro_rules! deps {
    () => {
        TaggedLen!();
    };
}

macro_rules! SmallVec {
    () => {
        deps!();
        # [repr (C)] pub struct SmallVec < T , const N : usize > { len : TaggedLen , raw : RawSmallVec < T , N > , _marker : PhantomData < T > , }
    };
}

SmallVec!()