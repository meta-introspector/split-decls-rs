macro_rules! deps {
    () => {
        ChunkBy!();
    };
}

macro_rules! GroupBy {
    () => {
        deps!();
        # [deprecated (note = "Use `ChunkBy` instead" , since = "0.13.0")] # [doc = " See [`ChunkBy`](crate::structs::ChunkBy)."] pub type GroupBy < K , I , F > = ChunkBy < K , I , F > ;
    };
}

GroupBy!()