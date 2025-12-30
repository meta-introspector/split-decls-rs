// Generated macro for impl_678 (impl)
macro_rules! Depcrate_stable_hasherimpl_678 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_678"}
// Dependencies: {}
impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for Box < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }
};
}
