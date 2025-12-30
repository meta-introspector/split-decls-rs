// Generated macro for impl_1179 (impl)
macro_rules! Depcrate_read_xcoff_symbolimpl_1179 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"impl_1179"}
// Dependencies: {}
impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > ObjectSymbolTable < 'data > for XcoffSymbolTable < 'data , 'file , Xcoff , R > { type Symbol = XcoffSymbol < 'data , 'file , Xcoff , R > ; type SymbolIterator = XcoffSymbolIterator < 'data , 'file , Xcoff , R > ; fn symbols (& self) -> Self :: SymbolIterator { XcoffSymbolIterator { file : self . file , symbols : self . symbols . iter () , } } fn symbol_by_index (& self , index : SymbolIndex) -> read :: Result < Self :: Symbol > { let symbol = self . symbols . symbol (index) ? ; Ok (XcoffSymbol { file : self . file , symbols : self . symbols , index , symbol , }) } }
};
}
