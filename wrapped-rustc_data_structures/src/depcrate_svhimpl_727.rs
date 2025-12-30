// Generated macro for impl_727 (impl)
macro_rules! Depcrate_svhimpl_727 {
() => {
// Module: crate::svh
// Provides: {"impl_727"}
// Dependencies: {}
impl < T > stable_hasher :: HashStable < T > for Svh { # [inline] fn hash_stable (& self , ctx : & mut T , hasher : & mut stable_hasher :: StableHasher) { let Svh { hash } = * self ; hash . hash_stable (ctx , hasher) ; } }
};
}
