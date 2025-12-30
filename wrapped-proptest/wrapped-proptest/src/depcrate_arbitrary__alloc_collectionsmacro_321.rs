// Generated macro for macro_321 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsmacro_321 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"macro_321"}
// Dependencies: {}
# [cfg (feature = "std")] arbitrary ! ([A : Arbitrary + Hash + Eq , B : Arbitrary] hash_map :: IntoIter < A , B >, SMapped < HashMap < A , B >, Self >, < HashMap < A , B > as Arbitrary >:: Parameters ; args => static_map (any_with ::< HashMap < A , B >> (args) , HashMap :: into_iter)) ;
};
}
