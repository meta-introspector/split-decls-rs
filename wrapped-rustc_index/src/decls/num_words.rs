macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! num_words {
    () => {
        deps!();
        # [inline] fn num_words < T : Idx > (domain_size : T) -> usize { domain_size . index () . div_ceil (WORD_BITS) }
    };
}

num_words!();