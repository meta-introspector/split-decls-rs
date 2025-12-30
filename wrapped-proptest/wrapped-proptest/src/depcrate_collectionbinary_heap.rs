// Generated macro for binary_heap (function)
macro_rules! Depcrate_collectionbinary_heap {
() => {
// Module: crate::collection
// Provides: {"binary_heap"}
// Dependencies: {}
# [doc = " Create a strategy to generate `BinaryHeap`s containing elements drawn from"] # [doc = " `element` and with a size range given by `size`."] pub fn binary_heap < T : Strategy > (element : T , size : impl Into < SizeRange > ,) -> BinaryHeapStrategy < T > where T :: Value : Ord , { BinaryHeapStrategy (statics :: Map :: new (vec (element , size) , VecToBinHeap)) }
};
}
