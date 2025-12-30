// Generated macro for impl_111 (impl)
macro_rules! Depcrate_string_cowimpl_111 {
() => {
// Module: crate::string_cow
// Provides: {"impl_111"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's StdString > for KStringCowBase < 's , B > { # [inline] fn from (other : & 's StdString) -> Self { Self :: from_ref (other . as_str ()) } }
};
}
