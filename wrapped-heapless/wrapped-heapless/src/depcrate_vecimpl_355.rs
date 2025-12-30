// Generated macro for impl_355 (impl)
macro_rules! Depcrate_vecimpl_355 {
() => {
// Module: crate::vec
// Provides: {"impl_355"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > Extend < T > for VecInner < T , LenT , S > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { self . extend (iter) ; } }
};
}
