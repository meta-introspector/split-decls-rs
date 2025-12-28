macro_rules! deps {
    () => {
        RawTable!();
        RawIterHash!();
        RawIterHashInner!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < T > RawIterHash < T > { # [cfg_attr (feature = "inline-more" , inline)] unsafe fn new < A : Allocator > (table : & RawTable < T , A > , hash : u64) -> Self { RawIterHash { inner : RawIterHashInner :: new (& table . table , hash) , _marker : PhantomData , } } }
    };
}

impl_107!();