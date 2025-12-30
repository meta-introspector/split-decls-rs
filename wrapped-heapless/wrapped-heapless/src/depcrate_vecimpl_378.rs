// Generated macro for impl_378 (impl)
macro_rules! Depcrate_vecimpl_378 {
() => {
// Module: crate::vec
// Provides: {"impl_378"}
// Dependencies: {}
impl < A , B , LenTA , SA > PartialEq < [B] > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , SA : VecStorage < A > + ? Sized , { # [inline] fn eq (& self , other : & [B]) -> bool { self . as_slice () . eq (other) } }
};
}
