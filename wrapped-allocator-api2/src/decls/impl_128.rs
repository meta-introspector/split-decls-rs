macro_rules! deps {
    () => {
        Allocator!();
        IntoIter!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < T , A : Allocator > ExactSizeIterator for IntoIter < T , A > { }
    };
}

impl_128!();