macro_rules! deps {
    () => {
        ImageNtHeaders64!();
        PeSegmentIterator!();
    };
}

macro_rules! PeSegmentIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the loadable sections in a [`PeFile64`](super::PeFile64)."] pub type PeSegmentIterator64 < 'data , 'file , R = & 'data [u8] > = PeSegmentIterator < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
    };
}

PeSegmentIterator64!()