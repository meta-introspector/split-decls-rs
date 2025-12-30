// Generated macro for impl_133 (impl)
macro_rules! Depcrate_string_refimpl_133 {
() => {
// Module: crate::string_ref
// Provides: {"impl_133"}
// Dependencies: {}
impl < 's > PartialEq < & 's str > for KStringRef < 's > { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . as_str () , * other) } }
};
}
