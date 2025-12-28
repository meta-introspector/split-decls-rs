macro_rules! deps {
    () => {
        FileHeader64!();
        XcoffSymbolTable!();
    };
}

macro_rules! XcoffSymbolTable64 {
    () => {
        deps!();
        # [doc = " A symbol table in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSymbolTable64 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolTable < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSymbolTable64!();