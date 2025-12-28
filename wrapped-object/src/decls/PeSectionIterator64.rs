macro_rules! deps {
    () => {
        PeSectionIterator!();
        ImageNtHeaders64!();
    };
}

macro_rules! PeSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`PeFile64`](super::PeFile64)."] pub type PeSectionIterator64 < 'data , 'file , R = & 'data [u8] > = PeSectionIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
    };
}

PeSectionIterator64!()