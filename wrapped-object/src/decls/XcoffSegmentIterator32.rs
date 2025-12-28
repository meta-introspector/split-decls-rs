macro_rules! deps {
    () => {
        XcoffSegmentIterator!();
        FileHeader32!();
    };
}

macro_rules! XcoffSegmentIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the segments in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSegmentIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffSegmentIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffSegmentIterator32!();