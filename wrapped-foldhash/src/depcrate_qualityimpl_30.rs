// Generated macro for impl_30 (impl)
macro_rules! Depcrate_qualityimpl_30 {
() => {
// Module: crate::quality
// Provides: {"impl_30"}
// Dependencies: {}
impl BuildHasher for SeedableRandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher { inner : self . inner . build_hasher () , } } }
};
}
