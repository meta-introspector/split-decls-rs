macro_rules! deps {
    () => {
        XcoffSection!();
        FileHeader32!();
    };
}

macro_rules! XcoffSection32 {
    () => {
        deps!();
        # [doc = " A section in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSection32 < 'data , 'file , R = & 'data [u8] > = XcoffSection < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffSection32!()