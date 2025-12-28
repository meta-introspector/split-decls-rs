macro_rules! EMPTY_U16 {
    () => {
        const EMPTY_U16 : & ZeroSlice < u16 > = zeroslice ! [] ;
    };
}

EMPTY_U16!()