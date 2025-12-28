macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! max_bit {
    () => {
        deps!();
        # [inline] fn max_bit (word : Word) -> usize { WORD_BITS - 1 - word . leading_zeros () as usize }
    };
}

max_bit!()