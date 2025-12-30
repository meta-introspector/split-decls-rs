// Generated macro for impl_94 (impl)
macro_rules! Depcrate_string_cowimpl_94 {
() => {
// Module: crate::string_cow
// Provides: {"impl_94"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialEq < String > for KStringCowBase < '_ , B > { # [inline] fn eq (& self , other : & StdString) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
