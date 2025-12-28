macro_rules! ChunksExactProducer {
    () => {
        struct ChunksExactProducer < 'data , T : Sync > { chunk_size : usize , slice : & 'data [T] , }
    };
}

ChunksExactProducer!()