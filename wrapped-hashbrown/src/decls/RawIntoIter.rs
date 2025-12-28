macro_rules! deps {
    () => {
        RawIter!();
    };
}

macro_rules! RawIntoIter {
    () => {
        deps!();
        # [doc = " Iterator which consumes a table and returns elements."] pub struct RawIntoIter < T , A : Allocator = Global > { iter : RawIter < T > , allocation : Option < (NonNull < u8 > , Layout , A) > , marker : PhantomData < T > , }
    };
}

RawIntoIter!()