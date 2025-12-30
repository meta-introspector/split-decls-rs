// Generated macro for impl_767 (impl)
macro_rules! Depcrate_collections_btree_setimpl_767 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_767"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T : Ord , A : Allocator + Clone > Iterator for Difference < 'a , T , A > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { match & mut self . inner { DifferenceInner :: Stitch { self_iter , other_iter } => { let mut self_next = self_iter . next () ? ; loop { match other_iter . peek () . map_or (Less , | other_next | self_next . cmp (other_next)) { Less => return Some (self_next) , Equal => { self_next = self_iter . next () ? ; other_iter . next () ; } Greater => { other_iter . next () ; } } } } DifferenceInner :: Search { self_iter , other_set } => loop { let self_next = self_iter . next () ? ; if ! other_set . contains (& self_next) { return Some (self_next) ; } } , DifferenceInner :: Iterate (iter) => iter . next () , } } fn size_hint (& self) -> (usize , Option < usize >) { let (self_len , other_len) = match & self . inner { DifferenceInner :: Stitch { self_iter , other_iter } => { (self_iter . len () , other_iter . len ()) } DifferenceInner :: Search { self_iter , other_set } => (self_iter . len () , other_set . len ()) , DifferenceInner :: Iterate (iter) => (iter . len () , 0) , } ; (self_len . saturating_sub (other_len) , Some (self_len)) } fn min (mut self) -> Option < & 'a T > { self . next () } }
};
}
