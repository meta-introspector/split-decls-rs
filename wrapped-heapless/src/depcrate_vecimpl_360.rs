// Generated macro for impl_360 (impl)
macro_rules! Depcrate_vecimpl_360 {
() => {
// Module: crate::vec
// Provides: {"impl_360"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize > FromIterator < T > for Vec < T , N , LenT > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { let mut vec = Self :: new () ; for i in iter { vec . push (i) . ok () . expect ("Vec::from_iter overflow") ; } vec } }
};
}
