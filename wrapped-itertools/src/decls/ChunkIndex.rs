macro_rules! deps {
    () => {
        IntoChunks!();
    };
}

macro_rules! ChunkIndex {
    () => {
        deps!();
        # [doc = " `ChunkIndex` acts like the grouping key function for `IntoChunks`"] # [derive (Debug , Clone)] struct ChunkIndex { size : usize , index : usize , key : usize , }
    };
}

ChunkIndex!()