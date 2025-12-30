// Generated macro for impl_380 (impl)
macro_rules! Depcrate_vecimpl_380 {
() => {
// Module: crate::vec
// Provides: {"impl_380"}
// Dependencies: {}
impl < A , B , LenTA , SA > PartialEq < & mut [B] > for VecInner < A , LenTA , SA > where A : PartialEq < B > , LenTA : LenType , SA : VecStorage < A > + ? Sized , { # [inline] fn eq (& self , other : & & mut [B]) -> bool { self . as_slice () . eq (* other) } }
};
}
