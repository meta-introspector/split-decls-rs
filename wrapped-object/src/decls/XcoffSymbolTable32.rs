macro_rules! deps {
    () => {
        FileHeader32!();
        XcoffSymbolTable!();
    };
}

macro_rules! XcoffSymbolTable32 {
    () => {
        deps!();
        # [doc = " A symbol table in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSymbolTable32 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolTable < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffSymbolTable32!();