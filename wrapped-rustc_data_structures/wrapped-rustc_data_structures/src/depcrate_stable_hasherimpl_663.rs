// Generated macro for impl_663 (impl)
macro_rules! Depcrate_stable_hasherimpl_663 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_663"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for f64 { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let val : u64 = self . to_bits () ; val . hash_stable (ctx , hasher) ; } }
};
}
