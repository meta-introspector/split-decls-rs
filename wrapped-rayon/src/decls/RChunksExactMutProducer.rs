macro_rules! RChunksExactMutProducer {
    () => {
        struct RChunksExactMutProducer < 'data , T : Send > { chunk_size : usize , slice : & 'data mut [T] , }
    };
}

RChunksExactMutProducer!()