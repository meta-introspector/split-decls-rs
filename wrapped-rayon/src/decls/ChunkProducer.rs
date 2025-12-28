macro_rules! ChunkProducer {
    () => {
        pub (super) struct ChunkProducer < P , F > { chunk_size : usize , len : usize , base : P , map : F , }
    };
}

ChunkProducer!();