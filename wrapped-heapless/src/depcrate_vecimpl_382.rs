// Generated macro for impl_382 (impl)
macro_rules! Depcrate_vecimpl_382 {
() => {
// Module: crate::vec
// Provides: {"impl_382"}
// Dependencies: {}
impl < T , LenTA : LenType , LenTB : LenType , SA : VecStorage < T > + ? Sized , SB : VecStorage < T > + ? Sized > PartialOrd < VecInner < T , LenTA , SA > > for VecInner < T , LenTB , SB > where T : PartialOrd , { fn partial_cmp (& self , other : & VecInner < T , LenTA , SA >) -> Option < Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
};
}
