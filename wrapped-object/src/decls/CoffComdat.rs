macro_rules! deps {
    () => {
        ImageFileHeader!();
        ReadRef!();
        CoffHeader!();
        SymbolIndex!();
        CoffFile!();
        ImageSymbol!();
        ObjectComdat!();
    };
}

macro_rules! CoffComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`CoffFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectComdat`] trait implementation."] # [derive (Debug)] pub struct CoffComdat < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { file : & 'file CoffFile < 'data , R , Coff > , symbol_index : SymbolIndex , symbol : & 'data Coff :: ImageSymbol , selection : u8 , }
    };
}

CoffComdat!();