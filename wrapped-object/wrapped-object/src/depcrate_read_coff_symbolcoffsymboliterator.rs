// Generated macro for CoffSymbolIterator (struct)
macro_rules! Depcrate_read_coff_symbolCoffSymbolIterator {
() => {
// Module: crate::read::coff::symbol
// Provides: {"CoffSymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for the symbols in a [`CoffFile`](super::CoffFile)"] # [doc = " or [`PeFile`](crate::read::pe::PeFile)."] pub struct CoffSymbolIterator < 'data , 'file , R = & 'data [u8] , Coff = pe :: ImageFileHeader > where R : ReadRef < 'data > , Coff : CoffHeader , { file : & 'file CoffCommon < 'data , R , Coff > , index : SymbolIndex , }
};
}
