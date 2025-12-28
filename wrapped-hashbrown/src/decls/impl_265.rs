macro_rules! deps {
    () => {
        IntoValues!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < K , V , A : Allocator > FusedIterator for IntoValues < K , V , A > { }
    };
}

impl_265!();