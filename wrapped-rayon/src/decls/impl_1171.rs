macro_rules! deps {
    () => {
        RChunks!();
    };
}

macro_rules! impl_1171 {
    () => {
        deps!();
        impl < 'data , T > RChunks < 'data , T > { pub (super) fn new (chunk_size : usize , slice : & 'data [T]) -> Self { Self { chunk_size , slice } } }
    };
}

impl_1171!();