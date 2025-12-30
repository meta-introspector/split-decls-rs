// Generated macro for impl_360 (impl)
macro_rules! Depcrate_read_coff_comdatimpl_360 {
() => {
// Module: crate::read::coff::comdat
// Provides: {"impl_360"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > CoffComdatIterator < 'data , 'file , R , Coff > { pub (crate) fn new (file : & 'file CoffFile < 'data , R , Coff >) -> Self { CoffComdatIterator { file , index : SymbolIndex (0) , } } }
};
}
