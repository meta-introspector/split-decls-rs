// Generated macro for impl_361 (impl)
macro_rules! Depcrate_read_coff_comdatimpl_361 {
() => {
// Module: crate::read::coff::comdat
// Provides: {"impl_361"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffComdatIterator < 'data , 'file , R , Coff > { type Item = CoffComdat < 'data , 'file , R , Coff > ; fn next (& mut self) -> Option < Self :: Item > { loop { let index = self . index ; let symbol = self . file . common . symbols . symbol (index) . ok () ? ; self . index . 0 += 1 + symbol . number_of_aux_symbols () as usize ; if let Some (comdat) = CoffComdat :: parse (self . file , symbol , index) { return Some (comdat) ; } } } }
};
}
