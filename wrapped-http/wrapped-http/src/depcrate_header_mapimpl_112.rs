// Generated macro for impl_112 (impl)
macro_rules! Depcrate_header_mapimpl_112 {
() => {
// Module: crate::header::map
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'a , T > Iterator for ValueDrain < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { if self . first . is_some () { self . first . take () } else if let Some (ref mut extras) = self . next { extras . next () } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { match (& self . first , & self . next) { (& Some (_) , & None) => (1 , Some (1)) , (& Some (_) , Some (extras)) => { let (l , u) = extras . size_hint () ; (l + 1 , u . map (| u | u + 1)) } (& None , Some (extras)) => extras . size_hint () , (& None , & None) => (0 , Some (0)) , } } }
};
}
