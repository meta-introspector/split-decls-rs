// Generated macro for impl_106 (impl)
macro_rules! Depcrate_string_cowimpl_106 {
() => {
// Module: crate::string_cow
// Provides: {"impl_106"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > From < KStringBase < B > > for KStringCowBase < '_ , B > { # [inline] fn from (other : KStringBase < B >) -> Self { let inner = KStringCowInner :: Owned (other) ; Self { inner } } }
};
}
