macro_rules! deps {
    () => {
        ReadRef!();
        SymbolTable!();
        CoffFile!();
        CoffHeader!();
        ImageFileHeader!();
        PeFile!();
        SectionTable!();
    };
}

macro_rules! CoffCommon {
    () => {
        deps!();
        # [doc = " The common parts of `PeFile` and `CoffFile`."] # [derive (Debug)] pub (crate) struct CoffCommon < 'data , R : ReadRef < 'data > , Coff : CoffHeader = pe :: ImageFileHeader > { pub (crate) sections : SectionTable < 'data > , pub (crate) symbols : SymbolTable < 'data , R , Coff > , pub (crate) image_base : u64 , }
    };
}

CoffCommon!()