// Generated macro for impl_131 (impl)
macro_rules! Depcrate_string_refimpl_131 {
() => {
// Module: crate::string_ref
// Provides: {"impl_131"}
// Dependencies: {}
impl < 's > PartialEq < KStringRef < 's > > for KStringRef < 's > { # [inline] fn eq (& self , other : & KStringRef < 's >) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
