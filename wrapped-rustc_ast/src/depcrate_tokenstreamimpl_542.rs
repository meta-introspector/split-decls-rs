// Generated macro for impl_542 (impl)
macro_rules! Depcrate_tokenstreamimpl_542 {
() => {
// Module: crate::tokenstream
// Provides: {"impl_542"}
// Dependencies: {}
impl < CTX > HashStable < CTX > for TokenStream where CTX : crate :: HashStableContext , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { for sub_tt in self . iter () { sub_tt . hash_stable (hcx , hasher) ; } } }
};
}
