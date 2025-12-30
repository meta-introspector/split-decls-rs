// Generated macro for JoinInput (trait)
macro_rules! Depcrate_joinJoinInput {
() => {
// Module: crate::join
// Provides: {"JoinInput"}
// Dependencies: {}
# [doc = " An input that can be used with `from_join`; either a `Variable` or a `Relation`."] pub trait JoinInput < 'me , Tuple : Ord > : Copy { # [doc = " If we are on iteration N of the loop, these are the tuples"] # [doc = " added on iteration N-1. (For a `Relation`, this is always an"] # [doc = " empty slice.)"] type RecentTuples : Deref < Target = [Tuple] > ; # [doc = " If we are on iteration N of the loop, these are the tuples"] # [doc = " added on iteration N - 2 or before. (For a `Relation`, this is"] # [doc = " just `self`.)"] type StableTuples : Deref < Target = [Relation < Tuple >] > ; # [doc = " Get the set of recent tuples."] fn recent (self) -> Self :: RecentTuples ; # [doc = " Get the set of stable tuples."] fn stable (self) -> Self :: StableTuples ; }
};
}
