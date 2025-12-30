// Generated macro for impl_698 (impl)
macro_rules! Depcrate_stable_hasherimpl_698 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_698"}
// Dependencies: {}
impl < R : Idx , C : Idx , CTX > HashStable < CTX > for bit_set :: BitMatrix < R , C > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
};
}
