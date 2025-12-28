macro_rules! deps {
    () => {
        ChunkId!();
    };
}

macro_rules! COMMIT_DATA_CHUNK_ID {
    () => {
        deps!();
        const COMMIT_DATA_CHUNK_ID : ChunkId = * b"CDAT" ;
    };
}

COMMIT_DATA_CHUNK_ID!();