// Generated macro for Position (struct)
macro_rules! Depcrate_basePosition {
() => {
// Module: crate::base
// Provides: {"Position"}
// Dependencies: {}
# [doc = " A search result."] # [doc = ""] # [doc = " The result indicates whether the key was found, as well as what were the adjacent nodes to the"] # [doc = " key on each level of the skip list."] struct Position < 'a , K , V > { # [doc = " Reference to a node with the given key, if found."] # [doc = ""] # [doc = " If this is `Some` then it will point to the same node as `right[0]`."] found : Option < & 'a Node < K , V > > , # [doc = " Adjacent nodes with smaller keys (predecessors)."] left : [& 'a Tower < K , V > ; MAX_HEIGHT] , # [doc = " Adjacent nodes with equal or greater keys (successors)."] right : [Shared < 'a , Node < K , V > > ; MAX_HEIGHT] , }
};
}
