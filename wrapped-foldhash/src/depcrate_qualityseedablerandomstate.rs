// Generated macro for SeedableRandomState (struct)
macro_rules! Depcrate_qualitySeedableRandomState {
() => {
// Module: crate::quality
// Provides: {"SeedableRandomState"}
// Dependencies: {}
# [doc = " A [`BuildHasher`] for [`quality::FoldHasher`](FoldHasher) that is randomly"] # [doc = " initialized by default, but can also be initialized with a specific seed."] # [doc = ""] # [doc = " This can be useful for e.g. testing, but the downside is that this type"] # [doc = " has a size of 16 bytes rather than the 8 bytes [`RandomState`] is."] # [derive (Clone , Default , Debug)] pub struct SeedableRandomState { inner : fast :: SeedableRandomState , }
};
}
