// Generated macro for impl_27 (impl)
macro_rules! Depcrate_qualityimpl_27 {
() => {
// Module: crate::quality
// Provides: {"impl_27"}
// Dependencies: {}
impl BuildHasher for RandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher { inner : self . inner . build_hasher () , } } }
};
}
