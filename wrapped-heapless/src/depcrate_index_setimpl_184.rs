// Generated macro for impl_184 (impl)
macro_rules! Depcrate_index_setimpl_184 {
() => {
// Module: crate::index_set
// Provides: {"impl_184"}
// Dependencies: {}
impl < 'a , T , S , const N : usize > Iterator for Intersection < 'a , T , S , N > where S : BuildHasher , T : Eq + Hash , { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { loop { let elt = self . iter . next () ? ; if self . other . contains (elt) { return Some (elt) ; } } } }
};
}
