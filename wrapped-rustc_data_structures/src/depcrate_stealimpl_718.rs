// Generated macro for impl_718 (impl)
macro_rules! Depcrate_stealimpl_718 {
() => {
// Module: crate::steal
// Provides: {"impl_718"}
// Dependencies: {}
impl < CTX , T : HashStable < CTX > > HashStable < CTX > for Steal < T > { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . borrow () . hash_stable (hcx , hasher) ; } }
};
}
