macro_rules! deps {
    () => {
        FileHeader32!();
        XcoffSegment!();
    };
}

macro_rules! XcoffSegment32 {
    () => {
        deps!();
        # [doc = " A segment in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSegment32 < 'data , 'file , R = & 'data [u8] > = XcoffSegment < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffSegment32!()