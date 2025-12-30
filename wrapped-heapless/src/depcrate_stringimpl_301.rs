// Generated macro for impl_301 (impl)
macro_rules! Depcrate_stringimpl_301 {
() => {
// Module: crate::string
// Provides: {"impl_301"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > PartialEq < StringInner < LenT , S > > for str { # [inline] fn eq (& self , other : & StringInner < LenT , S >) -> bool { Self :: eq (self , & other [..]) } }
};
}
