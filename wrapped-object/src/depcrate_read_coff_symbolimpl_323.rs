// Generated macro for impl_323 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_323 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_323"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > ObjectSymbolTable < 'data > for CoffSymbolTable < 'data , 'file , R , Coff > { type Symbol = CoffSymbol < 'data , 'file , R , Coff > ; type SymbolIterator = CoffSymbolIterator < 'data , 'file , R , Coff > ; fn symbols (& self) -> Self :: SymbolIterator { CoffSymbolIterator :: new (self . file) } fn symbol_by_index (& self , index : SymbolIndex) -> Result < Self :: Symbol > { let symbol = self . file . symbols . symbol (index) ? ; Ok (CoffSymbol { file : self . file , index , symbol , }) } }
};
}
