// Generated macro for impl_91 (impl)
macro_rules! Depcrate_string_cowimpl_91 {
() => {
// Module: crate::string_cow
// Provides: {"impl_91"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > PartialEq < KStringCowBase < 's , B > > for KStringCowBase < 's , B > { # [inline] fn eq (& self , other : & KStringCowBase < 's , B >) -> bool { PartialEq :: eq (self . as_str () , other . as_str ()) } }
};
}
