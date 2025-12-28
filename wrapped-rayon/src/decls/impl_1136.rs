macro_rules! deps {
    () => {
        ChunkBy!();
    };
}

macro_rules! impl_1136 {
    () => {
        deps!();
        impl < 'data , T , P > ChunkBy < 'data , T , P > { pub (super) fn new (slice : & 'data [T] , pred : P) -> Self { Self { pred , slice } } }
    };
}

impl_1136!();