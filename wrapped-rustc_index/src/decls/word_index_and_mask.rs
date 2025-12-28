macro_rules! deps {
    () => {
        Word!();
        Idx!();
    };
}

macro_rules! word_index_and_mask {
    () => {
        deps!();
        # [inline] fn word_index_and_mask < T : Idx > (elem : T) -> (usize , Word) { let elem = elem . index () ; let word_index = elem / WORD_BITS ; let mask = 1 << (elem % WORD_BITS) ; (word_index , mask) }
    };
}

word_index_and_mask!();