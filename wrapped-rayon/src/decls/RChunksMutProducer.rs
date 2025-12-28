macro_rules! RChunksMutProducer {
    () => {
        struct RChunksMutProducer < 'data , T : Send > { chunk_size : usize , slice : & 'data mut [T] , }
    };
}

RChunksMutProducer!();