macro_rules! deps {
    () => {
        ChunkId!();
    };
}

macro_rules! OID_FAN_CHUNK_ID {
    () => {
        deps!();
        const OID_FAN_CHUNK_ID : ChunkId = * b"OIDF" ;
    };
}

OID_FAN_CHUNK_ID!();