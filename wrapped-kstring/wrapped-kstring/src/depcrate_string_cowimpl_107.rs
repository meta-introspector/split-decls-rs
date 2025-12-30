// Generated macro for impl_107 (impl)
macro_rules! Depcrate_string_cowimpl_107 {
() => {
// Module: crate::string_cow
// Provides: {"impl_107"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's KStringBase < B > > for KStringCowBase < 's , B > { # [inline] fn from (other : & 's KStringBase < B >) -> Self { let other = other . as_ref () ; other . into () } }
};
}
