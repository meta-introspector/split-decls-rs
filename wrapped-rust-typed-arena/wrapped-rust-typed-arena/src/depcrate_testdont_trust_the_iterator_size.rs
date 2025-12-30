// Generated macro for dont_trust_the_iterator_size (function)
macro_rules! Depcrate_testdont_trust_the_iterator_size {
() => {
// Module: crate::test
// Provides: {"dont_trust_the_iterator_size"}
// Dependencies: {}
# [test] fn dont_trust_the_iterator_size () { use std :: iter :: repeat ; struct WrongSizeIter < I > (I) ; impl < I > Iterator for WrongSizeIter < I > where I : Iterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } } impl < I > ExactSizeIterator for WrongSizeIter < I > where I : Iterator { } let arena = Arena :: with_capacity (2) ; arena . alloc (0) ; let slice = arena . alloc_extend (WrongSizeIter (repeat (1) . take (1_000))) ; assert_eq ! (arena . chunks . borrow () . rest . len () , 1) ; assert_eq ! (slice . len () , 1000) ; }
};
}
