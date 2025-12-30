// Generated macro for macro_327 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsmacro_327 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"macro_327"}
// Dependencies: {}
lift1 ! ([, K : Ord + Arbitrary + 'static] BTreeMap < K , A >, RangedParams1 < K :: Parameters >; base , args => { let product_unpack ! [range , k] = args ; btree_map (any_with ::< K > (k) , base , range) }) ;
};
}
