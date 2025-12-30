// Generated macro for Merge (struct)
macro_rules! Depcrate_stream_merge_vecMerge {
() => {
// Module: crate::stream::merge::vec
// Provides: {"Merge"}
// Dependencies: {}
# [doc = " A stream that merges multiple streams into a single stream."] # [doc = ""] # [doc = " This `struct` is created by the [`merge`] method on the [`Merge`] trait. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`merge`]: trait.Merge.html#method.merge"] # [doc = " [`Merge`]: trait.Merge.html"] # [pin_project :: pin_project] pub struct Merge < S > where S : Stream , { # [pin] streams : Vec < S > , indexer : Indexer , complete : usize , wakers : WakerVec , state : PollVec , done : bool , }
};
}
