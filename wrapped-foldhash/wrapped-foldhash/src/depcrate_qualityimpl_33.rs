// Generated macro for impl_33 (impl)
macro_rules! Depcrate_qualityimpl_33 {
() => {
// Module: crate::quality
// Provides: {"impl_33"}
// Dependencies: {}
impl BuildHasher for FixedState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher { inner : self . inner . build_hasher () , } } }
};
}
