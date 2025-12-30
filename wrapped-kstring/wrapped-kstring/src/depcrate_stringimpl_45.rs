// Generated macro for impl_45 (impl)
macro_rules! Depcrate_stringimpl_45 {
() => {
// Module: crate::string
// Provides: {"impl_45"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialEq < & str > for KStringBase < B > { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . as_str () , * other) } }
};
}
