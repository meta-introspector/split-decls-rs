macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! macro_201 {
    () => {
        deps!();
        testconfig ! (search_rabinkarp_leftmost_first , PACKED_LEFTMOST_FIRST , | c : & mut Config | { c . only_rabin_karp (true) ; }) ;
    };
}

macro_201!();