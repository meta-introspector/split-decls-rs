macro_rules! deps {
    () => {
        ReadRef!();
        FileHeader!();
        SymbolBytes!();
        StringTable!();
    };
}

macro_rules! SymbolTable {
    () => {
        deps!();
        # [doc = " A table of symbol entries in an XCOFF file."] # [doc = ""] # [doc = " Also includes the string table used for the symbol names."] # [doc = ""] # [doc = " Returned by [`FileHeader::symbols`]."] # [derive (Debug)] pub struct SymbolTable < 'data , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { symbols : & 'data [xcoff :: SymbolBytes] , strings : StringTable < 'data , R > , header : PhantomData < Xcoff > , }
    };
}

SymbolTable!()