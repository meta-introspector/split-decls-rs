macro_rules! deps {
    () => {
        XcoffSegmentIterator!();
        FileHeader64!();
    };
}

macro_rules! XcoffSegmentIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSegmentIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffSegmentIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSegmentIterator64!();