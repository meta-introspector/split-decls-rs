// Generated macro for impl_44 (impl)
macro_rules! Depcrate_stringimpl_44 {
() => {
// Module: crate::string
// Provides: {"impl_44"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialEq < str > for KStringBase < B > { # [inline] fn eq (& self , other : & str) -> bool { PartialEq :: eq (self . as_str () , other) } }
};
}
