// Generated macro for macro_716 (macro)
macro_rules! Depcrate_collectionmacro_716 {
() => {
// Module: crate::collection
// Provides: {"macro_716"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy to create `VecDeque`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `vec_deque()` function in the same module."] # [derive (Clone , Debug)] pub struct VecDequeStrategy [< T >] [where T : Strategy] (statics :: Map < VecStrategy < T >, VecToDeque >) -> VecDequeValueTree < T :: Tree >; # [doc = " `ValueTree` corresponding to `VecDequeStrategy`."] # [derive (Clone , Debug)] pub struct VecDequeValueTree [< T >] [where T : ValueTree] (statics :: Map < VecValueTree < T >, VecToDeque >) -> VecDeque < T :: Value >; }
};
}
