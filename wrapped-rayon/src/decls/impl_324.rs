macro_rules! deps {
    () => {
        ChunkSeq!();
        Producer!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl < P > ExactSizeIterator for ChunkSeq < P > where P : Producer , { # [inline] fn len (& self) -> usize { self . len . div_ceil (self . chunk_size) } }
    };
}

impl_324!();