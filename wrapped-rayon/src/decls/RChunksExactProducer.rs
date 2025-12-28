macro_rules! RChunksExactProducer {
    () => {
        struct RChunksExactProducer < 'data , T : Sync > { chunk_size : usize , slice : & 'data [T] , }
    };
}

RChunksExactProducer!()