// Generated macro for direct_super_traits (function)
macro_rules! Depcrate_utilsdirect_super_traits {
() => {
// Module: crate::utils
// Provides: {"direct_super_traits"}
// Dependencies: {}
# [doc = " Returns an iterator over the direct super traits (including the trait itself)."] pub fn direct_super_traits (db : & dyn DefDatabase , trait_ : TraitId) -> SmallVec < [TraitId ; 4] > { let mut result = smallvec ! [trait_] ; direct_super_traits_cb (db , trait_ , | tt | { if ! result . contains (& tt) { result . push (tt) ; } }) ; result }
};
}
