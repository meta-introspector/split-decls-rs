// Generated macro for impl_110 (impl)
macro_rules! Depcrate_string_cowimpl_110 {
() => {
// Module: crate::string_cow
// Provides: {"impl_110"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > From < StdString > for KStringCowBase < '_ , B > { # [inline] fn from (other : StdString) -> Self { Self :: from_string (other) } }
};
}
