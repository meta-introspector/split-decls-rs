// Generated macro for impl_371 (impl)
macro_rules! Depcrate_vecimpl_371 {
() => {
// Module: crate::vec
// Provides: {"impl_371"}
// Dependencies: {}
impl < A , B , LenTB , const M : usize , SB > PartialEq < VecInner < B , LenTB , SB > > for [A ; M] where A : PartialEq < B > , LenTB : LenType , SB : VecStorage < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { self . eq (other . as_slice ()) } }
};
}
