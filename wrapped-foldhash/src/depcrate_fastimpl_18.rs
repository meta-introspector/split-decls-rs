// Generated macro for impl_18 (impl)
macro_rules! Depcrate_fastimpl_18 {
() => {
// Module: crate::fast
// Provides: {"impl_18"}
// Dependencies: {}
impl BuildHasher for FixedState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher :: with_seed (self . per_hasher_seed , SharedSeed :: global_fixed ()) } }
};
}
