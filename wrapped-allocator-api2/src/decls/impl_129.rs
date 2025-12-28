macro_rules! deps {
    () => {
        IntoIter!();
        Allocator!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < T , A : Allocator > FusedIterator for IntoIter < T , A > { }
    };
}

impl_129!()