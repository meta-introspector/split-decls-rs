macro_rules! deps {
    () => {
        RChunksMut!();
    };
}

macro_rules! impl_1185 {
    () => {
        deps!();
        impl < 'data , T > RChunksMut < 'data , T > { pub (super) fn new (chunk_size : usize , slice : & 'data mut [T]) -> Self { Self { chunk_size , slice } } }
    };
}

impl_1185!();