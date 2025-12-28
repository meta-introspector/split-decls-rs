macro_rules! deps {
    () => {
        CoffSegmentIterator!();
        AnonObjectHeaderBigobj!();
    };
}

macro_rules! CoffBigSegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the loadable sections in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSegmentIterator < 'data , 'file , R = & 'data [u8] > = CoffSegmentIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigSegmentIterator!()