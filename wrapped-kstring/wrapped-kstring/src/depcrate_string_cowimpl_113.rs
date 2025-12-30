// Generated macro for impl_113 (impl)
macro_rules! Depcrate_string_cowimpl_113 {
() => {
// Module: crate::string_cow
// Provides: {"impl_113"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's BoxedStr > for KStringCowBase < 's , B > { # [inline] fn from (other : & 's BoxedStr) -> Self { Self :: from_ref (other) } }
};
}
