// Generated macro for impl_34 (impl)
macro_rules! Depcrate_iteratorimpl_34 {
() => {
// Module: crate::iterator
// Provides: {"impl_34"}
// Dependencies: {}
impl < L , R > DoubleEndedIterator for Either < L , R > where L : DoubleEndedIterator , R : DoubleEndedIterator < Item = L :: Item > , { fn next_back (& mut self) -> Option < Self :: Item > { for_both ! (self , inner => inner . next_back ()) } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { for_both ! (self , inner => inner . nth_back (n)) } fn rfold < Acc , G > (self , init : Acc , f : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { for_both ! (self , inner => inner . rfold (init , f)) } fn rfind < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , { for_both ! (self , inner => inner . rfind (predicate)) } }
};
}
