macro_rules! deps {
    () => {
        ImageFileHeader!();
        ObjectSymbol!();
        ImageSymbol!();
        CoffCommon!();
        ReadRef!();
        CoffHeader!();
        SymbolIndex!();
    };
}

macro_rules! CoffSymbol {
    () => {
        deps!();
        # [doc = " A symbol in a [`CoffFile`](super::CoffFile) or [`PeFile`](crate::read::pe::PeFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] # [derive (Debug , Clone , Copy)] pub struct CoffSymbol < 'data , 'file , R = & 'data [u8] , Coff = pe :: ImageFileHeader > where R : ReadRef < 'data > , Coff : CoffHeader , { pub (crate) file : & 'file CoffCommon < 'data , R , Coff > , pub (crate) index : SymbolIndex , pub (crate) symbol : & 'data Coff :: ImageSymbol , }
    };
}

CoffSymbol!();