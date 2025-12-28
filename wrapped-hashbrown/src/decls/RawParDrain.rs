macro_rules! deps {
    () => {
        RawTable!();
    };
}

macro_rules! RawParDrain {
    () => {
        deps!();
        # [doc = " Parallel iterator which consumes elements without freeing the table storage."] pub struct RawParDrain < 'a , T , A : Allocator = Global > { table : NonNull < RawTable < T , A > > , marker : PhantomData < & 'a RawTable < T , A > > , }
    };
}

RawParDrain!();