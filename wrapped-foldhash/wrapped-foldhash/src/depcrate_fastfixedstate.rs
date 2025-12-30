// Generated macro for FixedState (struct)
macro_rules! Depcrate_fastFixedState {
() => {
// Module: crate::fast
// Provides: {"FixedState"}
// Dependencies: {}
# [doc = " A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed."] # [doc = ""] # [doc = " Not recommended unless you absolutely need determinism."] # [derive (Clone , Debug)] pub struct FixedState { per_hasher_seed : u64 , }
};
}
