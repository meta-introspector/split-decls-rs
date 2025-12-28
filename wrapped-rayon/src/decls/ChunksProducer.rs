macro_rules! ChunksProducer {
    () => {
        struct ChunksProducer < 'data , T : Sync > { chunk_size : usize , slice : & 'data [T] , }
    };
}

ChunksProducer!();