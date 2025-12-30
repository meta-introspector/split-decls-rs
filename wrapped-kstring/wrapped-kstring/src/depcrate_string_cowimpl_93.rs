// Generated macro for impl_93 (impl)
macro_rules! Depcrate_string_cowimpl_93 {
() => {
// Module: crate::string_cow
// Provides: {"impl_93"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > PartialEq < & 's str > for KStringCowBase < 's , B > { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . as_str () , * other) } }
};
}
