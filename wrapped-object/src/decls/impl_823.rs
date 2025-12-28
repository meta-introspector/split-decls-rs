macro_rules! deps {
    () => {
        XcoffSymbol!();
        ReadRef!();
        XcoffFile!();
        Symbol!();
        FileHeader!();
    };
}

macro_rules! impl_823 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > XcoffSymbol < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [doc = " Get the XCOFF file containing this symbol."] pub fn xcoff_file (& self) -> & 'file XcoffFile < 'data , Xcoff , R > { self . file } # [doc = " Get the raw XCOFF symbol structure."] pub fn xcoff_symbol (& self) -> & 'data Xcoff :: Symbol { self . symbol } }
    };
}

impl_823!()