// Generated macro for impl_661 (impl)
macro_rules! Depcrate_stable_hasherimpl_661 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_661"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for NonZero < usize > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . get () . hash_stable (ctx , hasher) } }
};
}
