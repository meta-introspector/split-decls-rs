macro_rules! EMPTY_CHAR {
    () => {
        const EMPTY_CHAR : & ZeroSlice < char > = zeroslice ! [] ;
    };
}

EMPTY_CHAR!()