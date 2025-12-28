macro_rules! RChunksProducer {
    () => {
        struct RChunksProducer < 'data , T : Sync > { chunk_size : usize , slice : & 'data [T] , }
    };
}

RChunksProducer!()