// Generated macro for CoffComdatSectionIterator (struct)
macro_rules! Depcrate_read_coff_comdatCoffComdatSectionIterator {
() => {
// Module: crate::read::coff::comdat
// Provides: {"CoffComdatSectionIterator"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`CoffFile`]."] # [derive (Debug)] pub struct CoffComdatSectionIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { file : & 'file CoffFile < 'data , R , Coff > , section_number : i32 , index : SymbolIndex , }
};
}
