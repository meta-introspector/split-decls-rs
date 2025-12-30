// Generated macro for impl_687 (impl)
macro_rules! Depcrate_stable_hasherimpl_687 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_687"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for bool { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (if * self { 1u8 } else { 0u8 }) . hash_stable (ctx , hasher) ; } }
};
}
