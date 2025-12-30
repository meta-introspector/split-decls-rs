// Generated macro for impl_699 (impl)
macro_rules! Depcrate_stable_hasherimpl_699 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_699"}
// Dependencies: {}
impl < T , CTX > HashStable < CTX > for bit_set :: FiniteBitSet < T > where T : HashStable < CTX > + bit_set :: FiniteBitSetTy , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }
};
}
