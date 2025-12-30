// Generated macro for macro_326 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsmacro_326 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"macro_326"}
// Dependencies: {}
arbitrary ! ([A : Arbitrary + Ord , B : Arbitrary] BTreeMap < A , B >, BTreeMapStrategy < A :: Strategy , B :: Strategy >, RangedParams2 < A :: Parameters , B :: Parameters >; args => { let product_unpack ! [range , a , b] = args ; btree_map (any_with ::< A > (a) , any_with ::< B > (b) , range) }) ;
};
}
