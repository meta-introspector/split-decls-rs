// Generated macro for impl_377 (impl)
macro_rules! Depcrate_vecimpl_377 {
() => {
// Module: crate::vec
// Provides: {"impl_377"}
// Dependencies: {}
impl < A , B , LenTA , SA , const N : usize > PartialEq < & [B ; N] > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , SA : VecStorage < A > + ? Sized , { # [inline] fn eq (& self , other : & & [B ; N]) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
