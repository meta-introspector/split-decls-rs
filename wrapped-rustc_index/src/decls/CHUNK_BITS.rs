macro_rules! CHUNK_BITS {
    () => {
        const CHUNK_BITS : usize = CHUNK_WORDS * WORD_BITS ;
    };
}

CHUNK_BITS!()