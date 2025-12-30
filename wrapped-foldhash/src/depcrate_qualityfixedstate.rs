// Generated macro for FixedState (struct)
macro_rules! Depcrate_qualityFixedState {
() => {
// Module: crate::quality
// Provides: {"FixedState"}
// Dependencies: {}
# [doc = " A [`BuildHasher`] for [`quality::FoldHasher`](FoldHasher) that always has the same fixed seed."] # [doc = ""] # [doc = " Not recommended unless you absolutely need determinism."] # [derive (Clone , Default , Debug)] pub struct FixedState { inner : fast :: FixedState , }
};
}
