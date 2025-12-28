macro_rules! deps {
    () => {
        CoffSectionIterator!();
        AnonObjectHeaderBigobj!();
    };
}

macro_rules! CoffBigSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSectionIterator < 'data , 'file , R = & 'data [u8] > = CoffSectionIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigSectionIterator!();