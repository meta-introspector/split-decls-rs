macro_rules! deps {
    () => {
        FileHeader64!();
        XcoffSegment!();
    };
}

macro_rules! XcoffSegment64 {
    () => {
        deps!();
        # [doc = " A segment in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSegment64 < 'data , 'file , R = & 'data [u8] > = XcoffSegment < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSegment64!()