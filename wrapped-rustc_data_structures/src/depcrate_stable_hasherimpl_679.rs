// Generated macro for impl_679 (impl)
macro_rules! Depcrate_stable_hasherimpl_679 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_679"}
// Dependencies: {}
impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for :: std :: rc :: Rc < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }
};
}
