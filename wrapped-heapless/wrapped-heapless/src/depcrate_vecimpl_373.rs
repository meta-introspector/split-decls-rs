// Generated macro for impl_373 (impl)
macro_rules! Depcrate_vecimpl_373 {
() => {
// Module: crate::vec
// Provides: {"impl_373"}
// Dependencies: {}
impl < A , B , LenTB , SB > PartialEq < VecInner < B , LenTB , SB > > for [A] where A : PartialEq < B > , LenTB : LenType , SB : VecStorage < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { self . eq (other . as_slice ()) } }
};
}
