macro_rules! deps {
    () => {
        XcoffSection!();
        FileHeader64!();
    };
}

macro_rules! XcoffSection64 {
    () => {
        deps!();
        # [doc = " A section in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSection64 < 'data , 'file , R = & 'data [u8] > = XcoffSection < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSection64!();