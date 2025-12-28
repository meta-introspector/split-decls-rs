macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < K , A : Allocator > FusedIterator for IntoIter < K , A > { }
    };
}

impl_432!()