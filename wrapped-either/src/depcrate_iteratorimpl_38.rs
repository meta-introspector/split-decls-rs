// Generated macro for impl_38 (impl)
macro_rules! Depcrate_iteratorimpl_38 {
() => {
// Module: crate::iterator
// Provides: {"impl_38"}
// Dependencies: {}
impl < L , R > DoubleEndedIterator for IterEither < L , R > where L : DoubleEndedIterator , R : DoubleEndedIterator , { fn next_back (& mut self) -> Option < Self :: Item > { Some (map_either ! (self . inner , ref mut inner => inner . next_back () ?)) } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { Some (map_either ! (self . inner , ref mut inner => inner . nth_back (n) ?)) } fn rfold < Acc , G > (self , init : Acc , f : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { wrap_either ! (self . inner => . rfold (init , f)) } fn rfind < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , { wrap_either ! (& mut self . inner => . rfind (predicate)) } }
};
}
