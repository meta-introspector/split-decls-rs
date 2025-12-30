// Generated macro for impl_32 (impl)
macro_rules! Depcrate_hasherimpl_32 {
() => {
// Module: crate::hasher
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "default-hasher")] impl BuildHasher for DefaultHashBuilder { type Hasher = DefaultHasher ; # [inline (always)] fn build_hasher (& self) -> Self :: Hasher { DefaultHasher { inner : self . inner . build_hasher () , } } }
};
}
