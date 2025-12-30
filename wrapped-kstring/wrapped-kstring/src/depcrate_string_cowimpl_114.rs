// Generated macro for impl_114 (impl)
macro_rules! Depcrate_string_cowimpl_114 {
() => {
// Module: crate::string_cow
// Provides: {"impl_114"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's str > for KStringCowBase < 's , B > { # [inline] fn from (other : & 's str) -> Self { Self :: from_ref (other) } }
};
}
