macro_rules! deps {
    () => {
        IntoIter!();
        Allocator!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < T , A : Allocator > ExactSizeIterator for IntoIter < T , A > { }
    };
}

impl_128!()