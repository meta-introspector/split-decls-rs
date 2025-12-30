// Generated macro for impl_372 (impl)
macro_rules! Depcrate_vecimpl_372 {
() => {
// Module: crate::vec
// Provides: {"impl_372"}
// Dependencies: {}
impl < A , B , LenTB , SB , const M : usize > PartialEq < VecInner < B , LenTB , SB > > for & [A ; M] where A : PartialEq < B > , LenTB : LenType , SB : VecStorage < B > , { fn eq (& self , other : & VecInner < B , LenTB , SB >) -> bool { (* self) . eq (other) } }
};
}
