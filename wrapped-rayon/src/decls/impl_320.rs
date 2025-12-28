macro_rules! deps {
    () => {
        ChunkProducer!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < P , F > ChunkProducer < P , F > { pub (super) fn new (chunk_size : usize , len : usize , base : P , map : F) -> Self { Self { chunk_size , len , base , map , } } }
    };
}

impl_320!();