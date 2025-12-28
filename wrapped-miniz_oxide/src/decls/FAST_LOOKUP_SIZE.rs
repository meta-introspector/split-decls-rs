macro_rules! FAST_LOOKUP_SIZE {
    () => {
        # [doc = " The size of the fast lookup table."] const FAST_LOOKUP_SIZE : u16 = 1 << FAST_LOOKUP_BITS ;
    };
}

FAST_LOOKUP_SIZE!();