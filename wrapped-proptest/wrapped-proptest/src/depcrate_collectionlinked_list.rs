// Generated macro for linked_list (function)
macro_rules! Depcrate_collectionlinked_list {
() => {
// Module: crate::collection
// Provides: {"linked_list"}
// Dependencies: {}
# [doc = " Create a strategy to generate `LinkedList`s containing elements drawn from"] # [doc = " `element` and with a size range given by `size`."] pub fn linked_list < T : Strategy > (element : T , size : impl Into < SizeRange > ,) -> LinkedListStrategy < T > { LinkedListStrategy (statics :: Map :: new (vec (element , size) , VecToLl)) }
};
}
