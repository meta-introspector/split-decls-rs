macro_rules! deps {
    () => {
        IntoKeys!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < K , V , A : Allocator > FusedIterator for IntoKeys < K , V , A > { }
    };
}

impl_259!();