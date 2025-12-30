// Generated macro for impl_370 (impl)
macro_rules! Depcrate_vecimpl_370 {
() => {
// Module: crate::vec
// Provides: {"impl_370"}
// Dependencies: {}
impl < A , B , LenTA , LenTB , SA , SB > PartialEq < VecInner < B , LenTB , SB > > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , LenTB : LenType , SA : VecStorage < A > + ? Sized , SB : VecStorage < B > + ? Sized , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
