// Generated macro for double_ended_iterator_methods (macro)
macro_rules! Depcrate_macrosdouble_ended_iterator_methods {
() => {
// Module: crate::macros
// Provides: {"double_ended_iterator_methods"}
// Dependencies: {}
macro_rules ! double_ended_iterator_methods { ($ map_elt : expr) => { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map ($ map_elt) } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { self . iter . nth_back (n) . map ($ map_elt) } } ; }
};
}
