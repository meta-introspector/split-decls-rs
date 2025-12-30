// Generated macro for macro_731 (macro)
macro_rules! Depcrate_collectionmacro_731 {
() => {
// Module: crate::collection
// Provides: {"macro_731"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy to create `BTreeSet`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `btree_set()` function in the same module."] # [derive (Clone , Debug)] pub struct BTreeSetStrategy [< T >] [where T : Strategy , T :: Value : Ord] (statics :: Filter < statics :: Map < VecStrategy < T >, VecToBTreeSet >, MinSize >) -> BTreeSetValueTree < T :: Tree >; # [doc = " `ValueTree` corresponding to `BTreeSetStrategy`."] # [derive (Clone , Debug)] pub struct BTreeSetValueTree [< T >] [where T : ValueTree , T :: Value : Ord] (statics :: Filter < statics :: Map < VecValueTree < T >, VecToBTreeSet >, MinSize >) -> BTreeSet < T :: Value >; }
};
}
