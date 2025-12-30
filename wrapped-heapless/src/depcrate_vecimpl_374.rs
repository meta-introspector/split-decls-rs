// Generated macro for impl_374 (impl)
macro_rules! Depcrate_vecimpl_374 {
() => {
// Module: crate::vec
// Provides: {"impl_374"}
// Dependencies: {}
impl < A , B , LenTB , SB > PartialEq < VecInner < B , LenTB , SB > > for & [A] where A : PartialEq < B > , LenTB : LenType , SB : VecStorage < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { (* self) . eq (other) } }
};
}
