macro_rules! ChunksExactMutProducer {
    () => {
        struct ChunksExactMutProducer < 'data , T : Send > { chunk_size : usize , slice : & 'data mut [T] , }
    };
}

ChunksExactMutProducer!()