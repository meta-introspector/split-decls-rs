macro_rules! deps {
    () => {
        ChunkId!();
    };
}

macro_rules! BASE_GRAPHS_LIST_CHUNK_ID {
    () => {
        deps!();
        const BASE_GRAPHS_LIST_CHUNK_ID : ChunkId = * b"BASE" ;
    };
}

BASE_GRAPHS_LIST_CHUNK_ID!();