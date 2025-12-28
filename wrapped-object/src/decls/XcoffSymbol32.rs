macro_rules! deps {
    () => {
        FileHeader32!();
        XcoffSymbol!();
    };
}

macro_rules! XcoffSymbol32 {
    () => {
        deps!();
        # [doc = " A symbol in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSymbol32 < 'data , 'file , R = & 'data [u8] > = XcoffSymbol < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffSymbol32!()