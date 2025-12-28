macro_rules! deps {
    () => {
        ChunkSize!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        const _ : () = assert ! (CHUNK_BITS <= ChunkSize :: MAX as usize) ;
    };
}

_!()