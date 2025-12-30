// Generated macro for macro_320 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsmacro_320 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"macro_320"}
// Dependencies: {}
# [cfg (feature = "std")] arbitrary ! ([A : Arbitrary + Hash + Eq , B : Arbitrary] HashMap < A , B >, HashMapStrategy < A :: Strategy , B :: Strategy >, RangedParams2 < A :: Parameters , B :: Parameters >; args => { let product_unpack ! [range , a , b] = args ; hash_map (any_with ::< A > (a) , any_with ::< B > (b) , range) }) ;
};
}
