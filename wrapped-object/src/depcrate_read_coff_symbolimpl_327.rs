// Generated macro for impl_327 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_327 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > fmt :: Debug for CoffSymbolIterator < 'data , 'file , R , Coff > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CoffSymbolIterator") . finish () } }
};
}
