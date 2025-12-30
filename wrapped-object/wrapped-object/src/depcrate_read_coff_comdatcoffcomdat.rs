// Generated macro for CoffComdat (struct)
macro_rules! Depcrate_read_coff_comdatCoffComdat {
() => {
// Module: crate::read::coff::comdat
// Provides: {"CoffComdat"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`CoffFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectComdat`] trait implementation."] # [derive (Debug)] pub struct CoffComdat < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { file : & 'file CoffFile < 'data , R , Coff > , symbol_index : SymbolIndex , symbol : & 'data Coff :: ImageSymbol , selection : u8 , }
};
}
