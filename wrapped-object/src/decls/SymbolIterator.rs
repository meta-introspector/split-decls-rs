macro_rules! deps {
    () => {
        FileHeader!();
        ReadRef!();
        SymbolTable!();
    };
}

macro_rules! SymbolIterator {
    () => {
        deps!();
        # [doc = " An iterator for symbol entries in an XCOFF file."] # [doc = ""] # [doc = " Yields the index and symbol structure for each symbol."] # [derive (Debug)] pub struct SymbolIterator < 'data , 'table , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { symbols : & 'table SymbolTable < 'data , Xcoff , R > , index : usize , }
    };
}

SymbolIterator!()