// Generated macro for impl_356 (impl)
macro_rules! Depcrate_vecimpl_356 {
() => {
// Module: crate::vec
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'a , T , LenT : LenType , S : VecStorage < T > + ? Sized > Extend < & 'a T > for VecInner < T , LenT , S > where T : 'a + Copy , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = & 'a T > , { self . extend (iter . into_iter () . cloned ()) ; } }
};
}
