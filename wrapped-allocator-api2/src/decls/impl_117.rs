macro_rules! deps {
    () => {
        Drain!();
        Allocator!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T , A : Allocator > ExactSizeIterator for Drain < '_ , T , A > { }
    };
}

impl_117!();