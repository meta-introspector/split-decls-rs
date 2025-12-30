// Generated macro for impl_148 (impl)
macro_rules! Depcrate_string_refimpl_148 {
() => {
// Module: crate::string_ref
// Provides: {"impl_148"}
// Dependencies: {}
impl < 's > From < & 's StdString > for KStringRef < 's > { # [inline] fn from (other : & 's StdString) -> Self { KStringRef :: from_ref (other . as_str ()) } }
};
}
