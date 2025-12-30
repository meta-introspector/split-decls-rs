// Generated macro for macro_727 (macro)
macro_rules! Depcrate_collectionmacro_727 {
() => {
// Module: crate::collection
// Provides: {"macro_727"}
// Dependencies: {}
opaque_strategy_wrapper ! { { # [cfg (feature = "std")] } { # [cfg_attr (docsrs , doc (cfg (feature = "std")))] } # [doc = " Strategy to create `HashSet`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `hash_set()` function in the same module."] # [derive (Clone , Debug)] pub struct HashSetStrategy [< T >] [where T : Strategy , T :: Value : Hash + Eq] (statics :: Filter < statics :: Map < VecStrategy < T >, VecToHashSet >, MinSize >) -> HashSetValueTree < T :: Tree >; # [doc = " `ValueTree` corresponding to `HashSetStrategy`."] # [derive (Clone , Debug)] pub struct HashSetValueTree [< T >] [where T : ValueTree , T :: Value : Hash + Eq] (statics :: Filter < statics :: Map < VecValueTree < T >, VecToHashSet >, MinSize >) -> HashSet < T :: Value >; }
};
}
