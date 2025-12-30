// Generated macro for SeedableRandomState (struct)
macro_rules! Depcrate_fastSeedableRandomState {
() => {
// Module: crate::fast
// Provides: {"SeedableRandomState"}
// Dependencies: {}
# [doc = " A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly"] # [doc = " initialized by default, but can also be initialized with a specific seed."] # [doc = ""] # [doc = " This can be useful for e.g. testing, but the downside is that this type"] # [doc = " has a size of 16 bytes rather than the 8 bytes [`RandomState`] is."] # [derive (Clone , Debug)] pub struct SeedableRandomState { per_hasher_seed : u64 , shared_seed : & 'static SharedSeed , }
};
}
