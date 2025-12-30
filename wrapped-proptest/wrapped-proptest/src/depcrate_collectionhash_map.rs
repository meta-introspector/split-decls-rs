// Generated macro for hash_map (function)
macro_rules! Depcrate_collectionhash_map {
() => {
// Module: crate::collection
// Provides: {"hash_map"}
// Dependencies: {}
# [doc = " Create a strategy to generate `HashMap`s containing keys and values drawn"] # [doc = " from `key` and `value` respectively, and with a size within the given"] # [doc = " range."] # [doc = ""] # [doc = " This strategy will implicitly do local rejects to ensure that the `HashMap`"] # [doc = " has at least the minimum number of elements, in case `key` should produce"] # [doc = " duplicate values."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn hash_map < K : Strategy , V : Strategy > (key : K , value : V , size : impl Into < SizeRange > ,) -> HashMapStrategy < K , V > where K :: Value : Hash + Eq , { let size = size . into () ; HashMapStrategy (statics :: Filter :: new (statics :: Map :: new (vec ((key , value) , size . clone ()) , VecToHashMap) , "HashMap minimum size" . into () , MinSize (size . start ()) ,)) }
};
}
