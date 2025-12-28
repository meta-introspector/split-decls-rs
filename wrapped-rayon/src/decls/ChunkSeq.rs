macro_rules! ChunkSeq {
    () => {
        pub (super) struct ChunkSeq < P > { chunk_size : usize , len : usize , inner : Option < P > , }
    };
}

ChunkSeq!();