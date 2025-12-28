macro_rules! deps {
    () => {
        Idx!();
        ChunkedBitSet!();
        ChunkedBitIter!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a , T : Idx > ChunkedBitIter < 'a , T > { # [inline] fn new (bit_set : & 'a ChunkedBitSet < T >) -> ChunkedBitIter < 'a , T > { ChunkedBitIter { bit_set , chunk_index : 0 , chunk_iter : bit_set . chunk_iter (0) } } }
    };
}

impl_31!();