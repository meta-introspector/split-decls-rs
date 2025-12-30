// Generated macro for impl_134 (impl)
macro_rules! Depcrate_string_refimpl_134 {
() => {
// Module: crate::string_ref
// Provides: {"impl_134"}
// Dependencies: {}
impl PartialEq < String > for KStringRef < '_ > { # [inline] fn eq (& self , other : & StdString) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
