macro_rules! deps {
    () => {
        ChunksMut!();
    };
}

macro_rules! impl_1158 {
    () => {
        deps!();
        impl < 'data , T > ChunksMut < 'data , T > { pub (super) fn new (chunk_size : usize , slice : & 'data mut [T]) -> Self { Self { chunk_size , slice } } }
    };
}

impl_1158!();