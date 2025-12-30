// Generated macro for ShuffleValueTree (struct)
macro_rules! Depcrate_strategy_shuffleShuffleValueTree {
() => {
// Module: crate::strategy::shuffle
// Provides: {"ShuffleValueTree"}
// Dependencies: {}
# [doc = " `ValueTree` shuffling adaptor."] # [doc = ""] # [doc = " See `Strategy::prop_shuffle()`."] # [derive (Clone , Debug)] pub struct ShuffleValueTree < V > { inner : V , rng : TestRng , # [doc = " The maximum amount to move any one element during shuffling."] # [doc = ""] # [doc = " This is `Cell` since we can't determine the bounds of the value until"] # [doc = " the first call to `current()`. (We technically _could_ by generating a"] # [doc = " value in `new_tree` and checking its length, but that would be a 100%"] # [doc = " slowdown.)"] dist : Cell < Option < num :: usize :: BinarySearch > > , # [doc = " Whether we've started simplifying `inner`. After this point, we can no"] # [doc = " longer simplify or complicate `dist`."] simplifying_inner : bool , }
};
}
