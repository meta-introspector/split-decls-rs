// Generated macro for impl_368 (impl)
macro_rules! Depcrate_vecimpl_368 {
() => {
// Module: crate::vec
// Provides: {"impl_368"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize > Drop for IntoIter < T , N , LenT > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (& mut self . vec . as_mut_slice () [self . next . into_usize () ..]) ; self . vec . len = LenT :: ZERO ; } } }
};
}
