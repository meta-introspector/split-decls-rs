macro_rules! deps {
    () => {
        ChunkedBitSet!();
        ChunkIter!();
        Idx!();
    };
}

macro_rules! ChunkedBitIter {
    () => {
        deps!();
        pub struct ChunkedBitIter < 'a , T : Idx > { bit_set : & 'a ChunkedBitSet < T > , chunk_index : usize , chunk_iter : ChunkIter < 'a > , }
    };
}

ChunkedBitIter!()