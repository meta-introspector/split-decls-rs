// Generated macro for impl_16 (impl)
macro_rules! Depcrate_fastimpl_16 {
() => {
// Module: crate::fast
// Provides: {"impl_16"}
// Dependencies: {}
impl FixedState { # [doc = " Creates a [`FixedState`] with the given per-hasher-seed."] # [inline (always)] pub const fn with_seed (per_hasher_seed : u64) -> Self { Self { per_hasher_seed : per_hasher_seed ^ ARBITRARY3 , } } }
};
}
