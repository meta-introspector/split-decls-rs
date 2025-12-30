// Generated macro for impl_300 (impl)
macro_rules! Depcrate_stringimpl_300 {
() => {
// Module: crate::string
// Provides: {"impl_300"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < & str > for StringInner < LenT , S > { # [inline] fn eq (& self , other : & & str) -> bool { str :: eq (self , & other [..]) } }
};
}
