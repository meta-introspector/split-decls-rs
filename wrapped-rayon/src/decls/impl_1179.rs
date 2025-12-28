macro_rules! deps {
    () => {
        RChunksExact!();
    };
}

macro_rules! impl_1179 {
    () => {
        deps!();
        impl < T > Clone for RChunksExact < '_ , T > { fn clone (& self) -> Self { RChunksExact { .. * self } } }
    };
}

impl_1179!();