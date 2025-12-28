macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl < T , A : Allocator > FusedIterator for Drain < '_ , T , A > { }
    };
}

impl_524!();