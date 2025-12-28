macro_rules! deps {
    () => {
        Idx!();
        BitIter!();
        ChunkedBitIter!();
    };
}

macro_rules! MixedBitIter {
    () => {
        deps!();
        pub enum MixedBitIter < 'a , T : Idx > { Small (BitIter < 'a , T >) , Large (ChunkedBitIter < 'a , T >) , }
    };
}

MixedBitIter!()