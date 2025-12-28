macro_rules! deps {
    () => {
        BitIter!();
    };
}

macro_rules! ChunkIter {
    () => {
        deps!();
        enum ChunkIter < 'a > { Zeros , Ones (Range < usize >) , Mixed (BitIter < 'a , usize >) , Finished , }
    };
}

ChunkIter!();