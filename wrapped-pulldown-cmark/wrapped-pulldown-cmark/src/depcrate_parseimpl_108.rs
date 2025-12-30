// Generated macro for impl_108 (impl)
macro_rules! Depcrate_parseimpl_108 {
() => {
// Module: crate::parse
// Provides: {"impl_108"}
// Dependencies: {}
impl CodeDelims { fn new () -> Self { Self { inner : Default :: default () , seen_first : false , } } fn insert (& mut self , count : usize , ix : TreeIndex) { if self . seen_first { self . inner . entry (count) . or_default () . push_back (ix) ; } else { self . seen_first = true ; } } fn is_populated (& self) -> bool { ! self . inner . is_empty () } fn find (& mut self , open_ix : TreeIndex , count : usize) -> Option < TreeIndex > { while let Some (ix) = self . inner . get_mut (& count) ? . pop_front () { if ix > open_ix { return Some (ix) ; } } None } fn clear (& mut self) { self . inner . clear () ; self . seen_first = false ; } }
};
}
