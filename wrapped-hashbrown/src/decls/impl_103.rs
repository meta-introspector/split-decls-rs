macro_rules! deps {
    () => {
        RawDrain!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < T , A : Allocator > ExactSizeIterator for RawDrain < '_ , T , A > { }
    };
}

impl_103!()