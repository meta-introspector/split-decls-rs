// Generated macro for vec (function)
macro_rules! Depcrate_collectionvec {
() => {
// Module: crate::collection
// Provides: {"vec"}
// Dependencies: {}
# [doc = " Create a strategy to generate `Vec`s containing elements drawn from"] # [doc = " `element` and with a size range given by `size`."] # [doc = ""] # [doc = " To make a `Vec` with a fixed number of elements, each with its own"] # [doc = " strategy, you can instead make a `Vec` of strategies (boxed if necessary)."] pub fn vec < T : Strategy > (element : T , size : impl Into < SizeRange > ,) -> VecStrategy < T > { let size = size . into () ; size . assert_nonempty () ; VecStrategy { element , size } }
};
}
