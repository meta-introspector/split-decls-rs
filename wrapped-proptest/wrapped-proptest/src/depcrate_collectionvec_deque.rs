// Generated macro for vec_deque (function)
macro_rules! Depcrate_collectionvec_deque {
() => {
// Module: crate::collection
// Provides: {"vec_deque"}
// Dependencies: {}
# [doc = " Create a strategy to generate `VecDeque`s containing elements drawn from"] # [doc = " `element` and with a size range given by `size`."] pub fn vec_deque < T : Strategy > (element : T , size : impl Into < SizeRange > ,) -> VecDequeStrategy < T > { VecDequeStrategy (statics :: Map :: new (vec (element , size) , VecToDeque)) }
};
}
