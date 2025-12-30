// Generated macro for macro_735 (macro)
macro_rules! Depcrate_collectionmacro_735 {
() => {
// Module: crate::collection
// Provides: {"macro_735"}
// Dependencies: {}
opaque_strategy_wrapper ! { { # [cfg (feature = "std")] } { # [cfg_attr (docsrs , doc (cfg (feature = "std")))] } # [doc = " Strategy to create `HashMap`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `hash_map()` function in the same module."] # [derive (Clone , Debug)] pub struct HashMapStrategy [< K , V >] [where K : Strategy , V : Strategy , K :: Value : Hash + Eq] (statics :: Filter < statics :: Map < VecStrategy < (K , V) >, VecToHashMap >, MinSize >) -> HashMapValueTree < K :: Tree , V :: Tree >; # [doc = " `ValueTree` corresponding to `HashMapStrategy`."] # [derive (Clone , Debug)] pub struct HashMapValueTree [< K , V >] [where K : ValueTree , V : ValueTree , K :: Value : Hash + Eq] (statics :: Filter < statics :: Map < VecValueTree < TupleValueTree < (K , V) >>, VecToHashMap >, MinSize >) -> HashMap < K :: Value , V :: Value >; }
};
}
