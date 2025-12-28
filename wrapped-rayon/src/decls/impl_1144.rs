macro_rules! deps {
    () => {
        Chunks!();
    };
}

macro_rules! impl_1144 {
    () => {
        deps!();
        impl < 'data , T > Chunks < 'data , T > { pub (super) fn new (chunk_size : usize , slice : & 'data [T]) -> Self { Self { chunk_size , slice } } }
    };
}

impl_1144!()