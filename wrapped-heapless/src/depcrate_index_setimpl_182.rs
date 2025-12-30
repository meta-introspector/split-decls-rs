// Generated macro for impl_182 (impl)
macro_rules! Depcrate_index_setimpl_182 {
() => {
// Module: crate::index_set
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a , T , S , const N : usize > Iterator for Difference < 'a , T , S , N > where S : BuildHasher , T : Eq + Hash , { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { loop { let elt = self . iter . next () ? ; if ! self . other . contains (elt) { return Some (elt) ; } } } }
};
}
