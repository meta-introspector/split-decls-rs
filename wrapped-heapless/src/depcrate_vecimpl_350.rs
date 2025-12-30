// Generated macro for impl_350 (impl)
macro_rules! Depcrate_vecimpl_350 {
() => {
// Module: crate::vec
// Provides: {"impl_350"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize , const M : usize > From < [T ; M] > for Vec < T , N , LenT > { fn from (array : [T ; M]) -> Self { Self :: from_array (array) } }
};
}
