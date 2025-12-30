// Generated macro for impl_1184 (impl)
macro_rules! Depcrate_read_xcoff_symbolimpl_1184 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"impl_1184"}
// Dependencies: {}
impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > Iterator for XcoffSymbolIterator < 'data , 'file , Xcoff , R > { type Item = XcoffSymbol < 'data , 'file , Xcoff , R > ; fn next (& mut self) -> Option < Self :: Item > { let (index , symbol) = self . symbols . next () ? ; Some (XcoffSymbol { file : self . file , symbols : self . symbols . symbols , index , symbol , }) } }
};
}
