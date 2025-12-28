macro_rules! ChunksMutProducer {
    () => {
        struct ChunksMutProducer < 'data , T : Send > { chunk_size : usize , slice : & 'data mut [T] , }
    };
}

ChunksMutProducer!();