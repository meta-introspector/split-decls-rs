// Generated macro for impl_40 (impl)
macro_rules! Depcrate_stable_hasherimpl_40 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_40"}
// Dependencies: {}
impl < H : ExtendedHasher + Default > Default for StableHasher < H > { # [doc = " Creates a new [`StableHasher`]."] # [doc = ""] # [doc = " To be used with the [`Hasher`] implementation and [`StableHasher::finish`]."] # [inline] fn default () -> Self { StableHasher { state : Default :: default () , } } }
};
}
