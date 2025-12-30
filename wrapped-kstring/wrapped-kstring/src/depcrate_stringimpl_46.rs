// Generated macro for impl_46 (impl)
macro_rules! Depcrate_stringimpl_46 {
() => {
// Module: crate::string
// Provides: {"impl_46"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialEq < String > for KStringBase < B > { # [inline] fn eq (& self , other : & StdString) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
