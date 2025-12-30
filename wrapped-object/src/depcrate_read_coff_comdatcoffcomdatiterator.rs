// Generated macro for CoffComdatIterator (struct)
macro_rules! Depcrate_read_coff_comdatCoffComdatIterator {
() => {
// Module: crate::read::coff::comdat
// Provides: {"CoffComdatIterator"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`CoffFile`]."] # [derive (Debug)] pub struct CoffComdatIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { file : & 'file CoffFile < 'data , R , Coff > , index : SymbolIndex , }
};
}
