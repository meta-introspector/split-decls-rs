macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! macro_193 {
    () => {
        deps!();
        testconfig ! (search_teddy_leftmost_first , PACKED_LEFTMOST_FIRST , | c : & mut Config | { c . only_teddy (true) ; }) ;
    };
}

macro_193!();