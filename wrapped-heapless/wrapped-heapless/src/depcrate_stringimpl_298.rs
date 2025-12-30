// Generated macro for impl_298 (impl)
macro_rules! Depcrate_stringimpl_298 {
() => {
// Module: crate::string
// Provides: {"impl_298"}
// Dependencies: {}
impl < LenT1 : LenType , LenT2 : LenType , S1 : StringStorage + ? Sized , S2 : StringStorage + ? Sized > PartialEq < StringInner < LenT1 , S1 > > for StringInner < LenT2 , S2 > { fn eq (& self , rhs : & StringInner < LenT1 , S1 >) -> bool { str :: eq (& * * self , & * * rhs) } }
};
}
