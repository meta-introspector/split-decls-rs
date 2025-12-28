macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl < K , V , A : Allocator > FusedIterator for Drain < '_ , K , V , A > { }
    };
}

impl_326!();