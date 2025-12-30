// Generated macro for CoffCommon (struct)
macro_rules! Depcrate_read_coff_fileCoffCommon {
() => {
// Module: crate::read::coff::file
// Provides: {"CoffCommon"}
// Dependencies: {}
# [doc = " The common parts of `PeFile` and `CoffFile`."] # [derive (Debug)] pub (crate) struct CoffCommon < 'data , R : ReadRef < 'data > , Coff : CoffHeader = pe :: ImageFileHeader > { pub (crate) sections : SectionTable < 'data > , pub (crate) symbols : SymbolTable < 'data , R , Coff > , pub (crate) image_base : u64 , }
};
}
