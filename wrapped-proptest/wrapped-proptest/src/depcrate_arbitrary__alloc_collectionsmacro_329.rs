// Generated macro for macro_329 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsmacro_329 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"macro_329"}
// Dependencies: {}
arbitrary ! ([A : Arbitrary + Ord , B : Arbitrary] btree_map :: IntoIter < A , B >, SMapped < BTreeMap < A , B >, Self >, < BTreeMap < A , B > as Arbitrary >:: Parameters ; args => static_map (any_with ::< BTreeMap < A , B >> (args) , BTreeMap :: into_iter)) ;
};
}
