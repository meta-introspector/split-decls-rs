// Generated macro for impl_1183 (impl)
macro_rules! Depcrate_read_xcoff_symbolimpl_1183 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"impl_1183"}
// Dependencies: {}
impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > fmt :: Debug for XcoffSymbolIterator < 'data , 'file , Xcoff , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("XcoffSymbolIterator") . finish () } }
};
}
