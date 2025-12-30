// Generated macro for impl_213 (impl)
macro_rules! Depcrate_read_anyimpl_213 {
() => {
// Module: crate::read::any
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SymbolIterator < 'data , 'file , R > { type Item = Symbol < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { map_inner_option_mut ! (self . inner , SymbolIteratorInternal , SymbolInternal , | iter | { iter . 0 . next () . map (| x | (x , PhantomData)) }) . map (| inner | Symbol { inner }) } }
};
}
