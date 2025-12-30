// Generated macro for hash_set (function)
macro_rules! Depcrate_collectionhash_set {
() => {
// Module: crate::collection
// Provides: {"hash_set"}
// Dependencies: {}
# [doc = " Create a strategy to generate `HashSet`s containing elements drawn from"] # [doc = " `element` and with a size range given by `size`."] # [doc = ""] # [doc = " This strategy will implicitly do local rejects to ensure that the `HashSet`"] # [doc = " has at least the minimum number of elements, in case `element` should"] # [doc = " produce duplicate values."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn hash_set < T : Strategy > (element : T , size : impl Into < SizeRange > ,) -> HashSetStrategy < T > where T :: Value : Hash + Eq , { let size = size . into () ; HashSetStrategy (statics :: Filter :: new (statics :: Map :: new (vec (element , size . clone ()) , VecToHashSet) , "HashSet minimum size" . into () , MinSize (size . start ()) ,)) }
};
}
