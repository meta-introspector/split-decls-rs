// Generated macro for impl_61 (impl)
macro_rules! Depcrate_stringimpl_61 {
() => {
// Module: crate::string
// Provides: {"impl_61"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's KStringCowBase < 's , B > > for KStringBase < B > { # [inline] fn from (other : & 's KStringCowBase < 's , B >) -> Self { other . clone () . into_owned () } }
};
}
