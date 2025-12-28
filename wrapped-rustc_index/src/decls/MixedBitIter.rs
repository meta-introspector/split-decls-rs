macro_rules! deps {
    () => {
        BitIter!();
        ChunkedBitIter!();
        Idx!();
    };
}

macro_rules! MixedBitIter {
    () => {
        deps!();
        pub enum MixedBitIter < 'a , T : Idx > { Small (BitIter < 'a , T >) , Large (ChunkedBitIter < 'a , T >) , }
    };
}

MixedBitIter!();