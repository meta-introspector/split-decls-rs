macro_rules! deps {
    () => {
        CoffComdatSectionIterator!();
        AnonObjectHeaderBigobj!();
    };
}

macro_rules! CoffBigComdatSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigComdatSectionIterator < 'data , 'file , R = & 'data [u8] > = CoffComdatSectionIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigComdatSectionIterator!()