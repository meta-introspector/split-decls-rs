macro_rules! deps {
    () => {
        RawTableInner!();
    };
}

macro_rules! RawTable {
    () => {
        deps!();
        # [doc = " A raw hash table with an unsafe API."] pub struct RawTable < T , A : Allocator = Global > { table : RawTableInner , alloc : A , marker : PhantomData < T > , }
    };
}

RawTable!();