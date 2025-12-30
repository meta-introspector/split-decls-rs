// Generated macro for impl_30 (impl)
macro_rules! Depcrate_astimpl_30 {
() => {
// Module: crate::ast
// Provides: {"impl_30"}
// Dependencies: {}
impl < CTX : rustc_span :: HashStableContext > HashStable < CTX > for Path { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . segments . len () . hash_stable (hcx , hasher) ; for segment in & self . segments { segment . ident . hash_stable (hcx , hasher) ; } } }
};
}
