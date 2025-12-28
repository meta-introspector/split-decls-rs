macro_rules! deps {
    () => {
        Drain!();
        Allocator!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < T , A : Allocator > FusedIterator for Drain < '_ , T , A > { }
    };
}

impl_118!();