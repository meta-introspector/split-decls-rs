// Generated macro for impl_60 (impl)
macro_rules! Depcrate_stringimpl_60 {
() => {
// Module: crate::string
// Provides: {"impl_60"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < KStringCowBase < 's , B > > for KStringBase < B > { # [inline] fn from (other : KStringCowBase < 's , B >) -> Self { other . into_owned () } }
};
}
