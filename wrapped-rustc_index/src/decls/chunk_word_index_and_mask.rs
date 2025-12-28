macro_rules! deps {
    () => {
        Idx!();
        Word!();
    };
}

macro_rules! chunk_word_index_and_mask {
    () => {
        deps!();
        # [inline] fn chunk_word_index_and_mask < T : Idx > (elem : T) -> (usize , Word) { let chunk_elem = elem . index () % CHUNK_BITS ; word_index_and_mask (chunk_elem) }
    };
}

chunk_word_index_and_mask!()