macro_rules! deps {
    () => {
        FileHeader32!();
        XcoffComdat!();
    };
}

macro_rules! XcoffComdat32 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffComdat32 < 'data , 'file , R = & 'data [u8] > = XcoffComdat < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffComdat32!();