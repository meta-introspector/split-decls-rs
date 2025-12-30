// Generated macro for SharedSeed (struct)
macro_rules! Depcrate_seedSharedSeed {
() => {
// Module: crate::seed
// Provides: {"SharedSeed"}
// Dependencies: {}
# [doc = " A random seed intended to be shared by many different foldhash instances."] # [doc = ""] # [doc = " This seed is consumed by [`FoldHasher::with_seed`](crate::fast::FoldHasher::with_seed),"] # [doc = " and [`SeedableRandomState::with_seed`](crate::fast::SeedableRandomState::with_seed)."] # [derive (Clone , Debug)] pub struct SharedSeed { pub (crate) seeds : [u64 ; 6] , }
};
}
