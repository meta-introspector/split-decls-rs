macro_rules! deps {
    () => {
        FileHeader64!();
        XcoffSegmentIterator!();
    };
}

macro_rules! XcoffSegmentIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the segments in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSegmentIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffSegmentIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSegmentIterator64!()