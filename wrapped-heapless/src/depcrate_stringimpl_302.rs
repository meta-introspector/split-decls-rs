// Generated macro for impl_302 (impl)
macro_rules! Depcrate_stringimpl_302 {
() => {
// Module: crate::string
// Provides: {"impl_302"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < StringInner < LenT , S > > for & str { # [inline] fn eq (& self , other : & StringInner < LenT , S >) -> bool { str :: eq (self , & other [..]) } }
};
}
