// Generated macro for impl_43 (impl)
macro_rules! Depcrate_stringimpl_43 {
() => {
// Module: crate::string
// Provides: {"impl_43"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialEq < KStringBase < B > > for KStringBase < B > { # [inline] fn eq (& self , other : & Self) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
