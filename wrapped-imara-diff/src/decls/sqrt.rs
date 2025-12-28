macro_rules! sqrt {
    () => {
        pub fn sqrt (val : usize) -> u32 { let nbits = (usize :: BITS - val . leading_zeros ()) / 2 ; 1 << nbits }
    };
}

sqrt!()