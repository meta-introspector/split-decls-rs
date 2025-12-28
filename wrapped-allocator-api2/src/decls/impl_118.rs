macro_rules! deps {
    () => {
        Allocator!();
        Drain!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < T , A : Allocator > FusedIterator for Drain < '_ , T , A > { }
    };
}

impl_118!()