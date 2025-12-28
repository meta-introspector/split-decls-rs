macro_rules! deps {
    () => {
        ReadRef!();
        CoffFile!();
        ImageFileHeader!();
        SymbolIndex!();
        CoffHeader!();
    };
}

macro_rules! CoffComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`CoffFile`]."] # [derive (Debug)] pub struct CoffComdatIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { file : & 'file CoffFile < 'data , R , Coff > , index : SymbolIndex , }
    };
}

CoffComdatIterator!()