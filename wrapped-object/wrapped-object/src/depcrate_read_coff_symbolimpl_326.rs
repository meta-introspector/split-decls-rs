// Generated macro for impl_326 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_326 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_326"}
// Dependencies: {}
impl < 'data , 'file , R , Coff > CoffSymbolIterator < 'data , 'file , R , Coff > where R : ReadRef < 'data > , Coff : CoffHeader , { pub (crate) fn new (file : & 'file CoffCommon < 'data , R , Coff >) -> Self { Self { file , index : SymbolIndex (0) , } } pub (crate) fn empty (file : & 'file CoffCommon < 'data , R , Coff >) -> Self { Self { file , index : SymbolIndex (file . symbols . len ()) , } } }
};
}
