macro_rules! deps {
    () => {
        ChunkId!();
    };
}

macro_rules! EXTENDED_EDGES_LIST_CHUNK_ID {
    () => {
        deps!();
        const EXTENDED_EDGES_LIST_CHUNK_ID : ChunkId = * b"EDGE" ;
    };
}

EXTENDED_EDGES_LIST_CHUNK_ID!();