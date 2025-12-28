macro_rules! deps {
    () => {
        PeSegmentIterator!();
        ImageNtHeaders32!();
    };
}

macro_rules! PeSegmentIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the loadable sections in a [`PeFile32`](super::PeFile32)."] pub type PeSegmentIterator32 < 'data , 'file , R = & 'data [u8] > = PeSegmentIterator < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
    };
}

PeSegmentIterator32!();