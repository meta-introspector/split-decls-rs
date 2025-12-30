// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < T > Hash for Index < T > { fn hash < StateHasher > (& self , hasher : & mut StateHasher) where StateHasher : Hasher , { self . index . hash (hasher) ; self . generation . hash (hasher) ; } }
};
}
