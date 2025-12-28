macro_rules! deps {
    () => {
        XcoffSymbol!();
        FileHeader64!();
    };
}

macro_rules! XcoffSymbol64 {
    () => {
        deps!();
        # [doc = " A symbol in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSymbol64 < 'data , 'file , R = & 'data [u8] > = XcoffSymbol < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSymbol64!();