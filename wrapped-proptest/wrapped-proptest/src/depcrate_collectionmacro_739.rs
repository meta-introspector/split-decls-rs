// Generated macro for macro_739 (macro)
macro_rules! Depcrate_collectionmacro_739 {
() => {
// Module: crate::collection
// Provides: {"macro_739"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy to create `BTreeMap`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `btree_map()` function in the same module."] # [derive (Clone , Debug)] pub struct BTreeMapStrategy [< K , V >] [where K : Strategy , V : Strategy , K :: Value : Ord] (statics :: Filter < statics :: Map < VecStrategy < (K , V) >, VecToBTreeMap >, MinSize >) -> BTreeMapValueTree < K :: Tree , V :: Tree >; # [doc = " `ValueTree` corresponding to `BTreeMapStrategy`."] # [derive (Clone , Debug)] pub struct BTreeMapValueTree [< K , V >] [where K : ValueTree , V : ValueTree , K :: Value : Ord] (statics :: Filter < statics :: Map < VecValueTree < TupleValueTree < (K , V) >>, VecToBTreeMap >, MinSize >) -> BTreeMap < K :: Value , V :: Value >; }
};
}
