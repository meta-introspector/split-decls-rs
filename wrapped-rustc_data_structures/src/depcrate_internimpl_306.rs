// Generated macro for impl_306 (impl)
macro_rules! Depcrate_internimpl_306 {
() => {
// Module: crate::intern
// Provides: {"impl_306"}
// Dependencies: {}
impl < 'a , T : Ord > Ord for Interned < 'a , T > { fn cmp (& self , other : & Interned < 'a , T >) -> Ordering { if ptr :: eq (self . 0 , other . 0) { Ordering :: Equal } else { let res = self . 0 . cmp (other . 0) ; debug_assert_ne ! (res , Ordering :: Equal) ; res } } }
};
}
