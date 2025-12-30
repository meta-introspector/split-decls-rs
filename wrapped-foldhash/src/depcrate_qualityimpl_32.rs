// Generated macro for impl_32 (impl)
macro_rules! Depcrate_qualityimpl_32 {
() => {
// Module: crate::quality
// Provides: {"impl_32"}
// Dependencies: {}
impl FixedState { # [doc = " Creates a [`FixedState`] with the given per-hasher seed."] # [inline (always)] pub const fn with_seed (per_hasher_seed : u64) -> Self { Self { inner : fast :: FixedState :: with_seed (folded_multiply (per_hasher_seed , ARBITRARY4)) , } } }
};
}
