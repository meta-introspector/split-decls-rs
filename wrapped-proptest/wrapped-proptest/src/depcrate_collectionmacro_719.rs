// Generated macro for macro_719 (macro)
macro_rules! Depcrate_collectionmacro_719 {
() => {
// Module: crate::collection
// Provides: {"macro_719"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy to create `LinkedList`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `linkedlist()` function in the same module."] # [derive (Clone , Debug)] pub struct LinkedListStrategy [< T >] [where T : Strategy] (statics :: Map < VecStrategy < T >, VecToLl >) -> LinkedListValueTree < T :: Tree >; # [doc = " `ValueTree` corresponding to `LinkedListStrategy`."] # [derive (Clone , Debug)] pub struct LinkedListValueTree [< T >] [where T : ValueTree] (statics :: Map < VecValueTree < T >, VecToLl >) -> LinkedList < T :: Value >; }
};
}
