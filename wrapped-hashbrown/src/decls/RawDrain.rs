macro_rules! deps {
    () => {
        RawTableInner!();
        RawIter!();
        RawTable!();
    };
}

macro_rules! RawDrain {
    () => {
        deps!();
        # [doc = " Iterator which consumes elements without freeing the table storage."] pub struct RawDrain < 'a , T , A : Allocator = Global > { iter : RawIter < T > , table : RawTableInner , orig_table : NonNull < RawTableInner > , marker : PhantomData < & 'a RawTable < T , A > > , }
    };
}

RawDrain!()