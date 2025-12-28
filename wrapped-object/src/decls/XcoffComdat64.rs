macro_rules! deps {
    () => {
        XcoffComdat!();
        FileHeader64!();
    };
}

macro_rules! XcoffComdat64 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffComdat64 < 'data , 'file , R = & 'data [u8] > = XcoffComdat < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffComdat64!()