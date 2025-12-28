macro_rules! deps {
    () => {
        FileHeader64!();
        XcoffSymbolIterator!();
    };
}

macro_rules! XcoffSymbolIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSymbolIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
    };
}

XcoffSymbolIterator64!()