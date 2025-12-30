// Generated macro for RandomState (struct)
macro_rules! Depcrate_fastRandomState {
() => {
// Module: crate::fast
// Provides: {"RandomState"}
// Dependencies: {}
# [doc = " A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly initialized."] # [derive (Clone , Debug)] pub struct RandomState { per_hasher_seed : u64 , global_seed : GlobalSeed , }
};
}
