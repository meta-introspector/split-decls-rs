// Generated macro for Head (struct)
macro_rules! Depcrate_baseHead {
() => {
// Module: crate::base
// Provides: {"Head"}
// Dependencies: {}
# [doc = " Tower at the head of a skip list."] # [doc = ""] # [doc = " This is located in the `SkipList` struct itself and holds a full height"] # [doc = " tower."] # [repr (C)] struct Head < K , V > { pointers : [Atomic < Node < K , V > > ; MAX_HEIGHT] , }
};
}
