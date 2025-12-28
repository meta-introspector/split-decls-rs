macro_rules! deps {
    () => {
        FileHeader!();
        XcoffFile!();
        ReadRef!();
        SymbolIterator!();
    };
}

macro_rules! XcoffSymbolIterator {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in an [`XcoffFile`]."] pub struct XcoffSymbolIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) symbols : SymbolIterator < 'data , 'file , Xcoff , R > , }
    };
}

XcoffSymbolIterator!();