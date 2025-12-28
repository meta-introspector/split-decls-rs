macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl < K , A : Allocator > FusedIterator for Drain < '_ , K , A > { }
    };
}

impl_436!();