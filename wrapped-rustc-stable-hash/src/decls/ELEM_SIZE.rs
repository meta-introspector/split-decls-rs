macro_rules! ELEM_SIZE {
    () => {
        const ELEM_SIZE : usize = mem :: size_of :: < u64 > () ;
    };
}

ELEM_SIZE!()