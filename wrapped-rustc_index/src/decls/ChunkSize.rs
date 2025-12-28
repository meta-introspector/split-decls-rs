macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! ChunkSize {
    () => {
        deps!();
        # [doc = " ChunkSize is small to keep `Chunk` small. The static assertion ensures it's"] # [doc = " not too small."] type ChunkSize = u16 ;
    };
}

ChunkSize!();