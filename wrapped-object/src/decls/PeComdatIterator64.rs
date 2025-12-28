macro_rules! deps {
    () => {
        PeComdatIterator!();
        PeFile64!();
        ImageNtHeaders64!();
    };
}

macro_rules! PeComdatIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`PeFile64`]."] pub type PeComdatIterator64 < 'data , 'file , R = & 'data [u8] > = PeComdatIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
    };
}

PeComdatIterator64!();