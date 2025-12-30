// Generated macro for impl_10 (impl)
macro_rules! Depcrate_fastimpl_10 {
() => {
// Module: crate::fast
// Provides: {"impl_10"}
// Dependencies: {}
impl BuildHasher for RandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher :: with_seed (self . per_hasher_seed , self . global_seed . get ()) } }
};
}
