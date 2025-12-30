// Generated macro for impl_92 (impl)
macro_rules! Depcrate_string_cowimpl_92 {
() => {
// Module: crate::string_cow
// Provides: {"impl_92"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialEq < str > for KStringCowBase < '_ , B > { # [inline] fn eq (& self , other : & str) -> bool { PartialEq :: eq (self . as_str () , other) } }
};
}
