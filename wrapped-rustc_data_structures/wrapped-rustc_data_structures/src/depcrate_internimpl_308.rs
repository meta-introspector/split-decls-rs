// Generated macro for impl_308 (impl)
macro_rules! Depcrate_internimpl_308 {
() => {
// Module: crate::intern
// Provides: {"impl_308"}
// Dependencies: {}
impl < T , CTX > HashStable < CTX > for Interned < '_ , T > where T : HashStable < CTX > , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }
};
}
