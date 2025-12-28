macro_rules! deps {
    () => {
        ObjectSymbol!();
        FileHeader!();
        Symbol!();
        XcoffFile!();
        ReadRef!();
        SymbolIndex!();
        SymbolTable!();
    };
}

macro_rules! XcoffSymbol {
    () => {
        deps!();
        # [doc = " A symbol in an [`XcoffFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] # [derive (Debug , Clone , Copy)] pub struct XcoffSymbol < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) symbols : & 'file SymbolTable < 'data , Xcoff , R > , pub (super) index : SymbolIndex , pub (super) symbol : & 'data Xcoff :: Symbol , }
    };
}

XcoffSymbol!()