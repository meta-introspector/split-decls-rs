macro_rules! deps {
    () => {
        CoffComdatIterator!();
        AnonObjectHeaderBigobj!();
    };
}

macro_rules! CoffBigComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigComdatIterator < 'data , 'file , R = & 'data [u8] > = CoffComdatIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigComdatIterator!()