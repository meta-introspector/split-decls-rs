// Generated macro for FlatPairs (struct)
macro_rules! Depcrate_iterators_flat_pairsFlatPairs {
() => {
// Module: crate::iterators::flat_pairs
// Provides: {"FlatPairs"}
// Dependencies: {}
# [doc = " An iterator over [`Pair`]s. It is created by [`Pairs::flatten`]."] # [doc = ""] # [doc = " [`Pair`]: struct.Pair.html"] # [doc = " [`Pairs::flatten`]: struct.Pairs.html#method.flatten"] pub struct FlatPairs < 'i , R > { queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , start : usize , end : usize , line_index : Rc < LineIndex > , }
};
}
