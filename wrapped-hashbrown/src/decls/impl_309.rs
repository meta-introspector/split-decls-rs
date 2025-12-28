macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < K , V , A : Allocator > FusedIterator for IntoIter < K , V , A > { }
    };
}

impl_309!()