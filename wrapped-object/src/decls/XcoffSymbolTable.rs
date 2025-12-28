macro_rules! deps {
    () => {
        XcoffFile!();
        SymbolTable!();
        FileHeader!();
        ReadRef!();
    };
}

macro_rules! XcoffSymbolTable {
    () => {
        deps!();
        # [doc = " A symbol table in an [`XcoffFile`]."] # [derive (Debug , Clone , Copy)] pub struct XcoffSymbolTable < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) symbols : & 'file SymbolTable < 'data , Xcoff , R > , }
    };
}

XcoffSymbolTable!()