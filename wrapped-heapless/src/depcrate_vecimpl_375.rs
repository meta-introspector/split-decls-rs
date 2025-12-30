// Generated macro for impl_375 (impl)
macro_rules! Depcrate_vecimpl_375 {
() => {
// Module: crate::vec
// Provides: {"impl_375"}
// Dependencies: {}
impl < A , B , LenTB : LenType , SB : VecStorage < B > > PartialEq < VecInner < B , LenTB , SB > > for & mut [A] where A : PartialEq < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { (* * self) . eq (other) } }
};
}
