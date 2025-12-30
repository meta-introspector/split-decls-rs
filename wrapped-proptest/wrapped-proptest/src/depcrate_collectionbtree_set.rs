// Generated macro for btree_set (function)
macro_rules! Depcrate_collectionbtree_set {
() => {
// Module: crate::collection
// Provides: {"btree_set"}
// Dependencies: {}
# [doc = " Create a strategy to generate `BTreeSet`s containing elements drawn from"] # [doc = " `element` and with a size range given by `size`."] # [doc = ""] # [doc = " This strategy will implicitly do local rejects to ensure that the"] # [doc = " `BTreeSet` has at least the minimum number of elements, in case `element`"] # [doc = " should produce duplicate values."] pub fn btree_set < T : Strategy > (element : T , size : impl Into < SizeRange > ,) -> BTreeSetStrategy < T > where T :: Value : Ord , { let size = size . into () ; BTreeSetStrategy (statics :: Filter :: new (statics :: Map :: new (vec (element , size . clone ()) , VecToBTreeSet) , "BTreeSet minimum size" . into () , MinSize (size . start ()) ,)) }
};
}
