macro_rules! deps {
    () => {
        ChunksExact!();
    };
}

macro_rules! impl_1152 {
    () => {
        deps!();
        impl < T > Clone for ChunksExact < '_ , T > { fn clone (& self) -> Self { ChunksExact { .. * self } } }
    };
}

impl_1152!()