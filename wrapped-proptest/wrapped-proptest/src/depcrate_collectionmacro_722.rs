// Generated macro for macro_722 (macro)
macro_rules! Depcrate_collectionmacro_722 {
() => {
// Module: crate::collection
// Provides: {"macro_722"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy to create `BinaryHeap`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `binary_heap()` function in the same module."] # [derive (Clone , Debug)] pub struct BinaryHeapStrategy [< T >] [where T : Strategy , T :: Value : Ord] (statics :: Map < VecStrategy < T >, VecToBinHeap >) -> BinaryHeapValueTree < T :: Tree >; # [doc = " `ValueTree` corresponding to `BinaryHeapStrategy`."] # [derive (Clone , Debug)] pub struct BinaryHeapValueTree [< T >] [where T : ValueTree , T :: Value : Ord] (statics :: Map < VecValueTree < T >, VecToBinHeap >) -> BinaryHeap < T :: Value >; }
};
}
