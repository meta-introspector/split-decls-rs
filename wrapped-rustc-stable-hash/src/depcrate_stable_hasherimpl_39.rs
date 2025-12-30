// Generated macro for impl_39 (impl)
macro_rules! Depcrate_stable_hasherimpl_39 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_39"}
// Dependencies: {}
impl < H : ExtendedHasher + Default > StableHasher < H > { # [doc = " Creates a new [`StableHasher`]."] # [doc = ""] # [doc = " To be used with the [`Hasher`] implementation and [`StableHasher::finish`]."] # [inline] pub fn new () -> Self { Default :: default () } }
};
}
