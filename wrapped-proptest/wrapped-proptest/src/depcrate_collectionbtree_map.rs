// Generated macro for btree_map (function)
macro_rules! Depcrate_collectionbtree_map {
() => {
// Module: crate::collection
// Provides: {"btree_map"}
// Dependencies: {}
# [doc = " Create a strategy to generate `BTreeMap`s containing keys and values drawn"] # [doc = " from `key` and `value` respectively, and with a size within the given"] # [doc = " range."] # [doc = ""] # [doc = " This strategy will implicitly do local rejects to ensure that the"] # [doc = " `BTreeMap` has at least the minimum number of elements, in case `key`"] # [doc = " should produce duplicate values."] pub fn btree_map < K : Strategy , V : Strategy > (key : K , value : V , size : impl Into < SizeRange > ,) -> BTreeMapStrategy < K , V > where K :: Value : Ord , { let size = size . into () ; BTreeMapStrategy (statics :: Filter :: new (statics :: Map :: new (vec ((key , value) , size . clone ()) , VecToBTreeMap) , "BTreeMap minimum size" . into () , MinSize (size . start ()) ,)) }
};
}
