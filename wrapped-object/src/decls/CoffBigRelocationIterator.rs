macro_rules! deps {
    () => {
        AnonObjectHeaderBigobj!();
        CoffRelocationIterator!();
    };
}

macro_rules! CoffBigRelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in a [`CoffBigSection`](super::CoffBigSection)."] pub type CoffBigRelocationIterator < 'data , 'file , R = & 'data [u8] > = CoffRelocationIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigRelocationIterator!();