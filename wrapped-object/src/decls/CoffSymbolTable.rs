macro_rules! deps {
    () => {
        ReadRef!();
        ImageFileHeader!();
        CoffCommon!();
        CoffHeader!();
    };
}

macro_rules! CoffSymbolTable {
    () => {
        deps!();
        # [doc = " A symbol table in a [`CoffFile`](super::CoffFile)"] # [doc = " or [`PeFile`](crate::read::pe::PeFile)."] # [derive (Debug , Clone , Copy)] pub struct CoffSymbolTable < 'data , 'file , R = & 'data [u8] , Coff = pe :: ImageFileHeader > where R : ReadRef < 'data > , Coff : CoffHeader , { pub (crate) file : & 'file CoffCommon < 'data , R , Coff > , }
    };
}

CoffSymbolTable!();