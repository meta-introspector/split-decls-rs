// Generated macro for impl_693 (impl)
macro_rules! Depcrate_stable_hasherimpl_693 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_693"}
// Dependencies: {}
impl < T , CTX > HashStable < CTX > for :: std :: mem :: Discriminant < T > { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
};
}
