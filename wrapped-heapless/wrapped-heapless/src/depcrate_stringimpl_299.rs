// Generated macro for impl_299 (impl)
macro_rules! Depcrate_stringimpl_299 {
() => {
// Module: crate::string
// Provides: {"impl_299"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < str > for StringInner < LenT , S > { # [inline] fn eq (& self , other : & str) -> bool { str :: eq (self , other) } }
};
}
