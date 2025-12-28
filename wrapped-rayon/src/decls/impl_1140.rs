macro_rules! deps {
    () => {
        ChunkByMut!();
    };
}

macro_rules! impl_1140 {
    () => {
        deps!();
        impl < 'data , T , P > ChunkByMut < 'data , T , P > { pub (super) fn new (slice : & 'data mut [T] , pred : P) -> Self { Self { pred , slice } } }
    };
}

impl_1140!()