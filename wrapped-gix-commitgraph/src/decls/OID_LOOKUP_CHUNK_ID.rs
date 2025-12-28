macro_rules! deps {
    () => {
        ChunkId!();
    };
}

macro_rules! OID_LOOKUP_CHUNK_ID {
    () => {
        deps!();
        const OID_LOOKUP_CHUNK_ID : ChunkId = * b"OIDL" ;
    };
}

OID_LOOKUP_CHUNK_ID!()