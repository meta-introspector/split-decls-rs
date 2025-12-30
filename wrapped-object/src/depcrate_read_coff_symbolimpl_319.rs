// Generated macro for impl_319 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_319 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_319"}
// Dependencies: {}
impl < 'data , 'table , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for SymbolIterator < 'data , 'table , R , Coff > { type Item = (SymbolIndex , & 'data Coff :: ImageSymbol) ; fn next (& mut self) -> Option < Self :: Item > { let index = self . index ; let symbol = self . symbols . symbol (index) . ok () ? ; self . index . 0 += 1 + symbol . number_of_aux_symbols () as usize ; Some ((index , symbol)) } }
};
}
