// Generated macro for impl_182 (impl)
macro_rules! Depcrate_linked_hash_setimpl_182 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a , T , S > Iterator for Difference < 'a , T , S > where T : Eq + Hash , S : BuildHasher , { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { loop { match self . iter . next () { None => return None , Some (elt) => { if ! self . other . contains (elt) { return Some (elt) ; } } } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}
