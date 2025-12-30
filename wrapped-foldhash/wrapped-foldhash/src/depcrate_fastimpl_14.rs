// Generated macro for impl_14 (impl)
macro_rules! Depcrate_fastimpl_14 {
() => {
// Module: crate::fast
// Provides: {"impl_14"}
// Dependencies: {}
impl BuildHasher for SeedableRandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher :: with_seed (self . per_hasher_seed , self . shared_seed) } }
};
}
