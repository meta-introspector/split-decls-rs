// Generated macro for impl_383 (impl)
macro_rules! Depcrate_vecimpl_383 {
() => {
// Module: crate::vec
// Provides: {"impl_383"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > Ord for VecInner < T , LenT , S > where T : Ord , { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . as_slice () . cmp (other . as_slice ()) } }
};
}
