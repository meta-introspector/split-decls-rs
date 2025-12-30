// Generated macro for Merge (struct)
macro_rules! Depcrate_stream_merge_arrayMerge {
() => {
// Module: crate::stream::merge::array
// Provides: {"Merge"}
// Dependencies: {}
# [doc = " A stream that merges multiple streams into a single stream."] # [doc = ""] # [doc = " This `struct` is created by the [`merge`] method on the [`Merge`] trait. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`merge`]: trait.Merge.html#method.merge"] # [doc = " [`Merge`]: trait.Merge.html"] # [pin_project :: pin_project] pub struct Merge < S , const N : usize > where S : Stream , { # [pin] streams : [S ; N] , indexer : Indexer , wakers : WakerArray < N > , state : PollArray < N > , complete : usize , done : bool , }
};
}
