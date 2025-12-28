macro_rules! deps {
    () => {
        XcoffSymbolIterator!();
        FileHeader32!();
    };
}

macro_rules! XcoffSymbolIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSymbolIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
    };
}

XcoffSymbolIterator32!()