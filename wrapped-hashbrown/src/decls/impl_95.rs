macro_rules! deps {
    () => {
        RawIntoIter!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < T , A : Allocator > ExactSizeIterator for RawIntoIter < T , A > { }
    };
}

impl_95!();