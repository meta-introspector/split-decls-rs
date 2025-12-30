// Generated macro for impl_147 (impl)
macro_rules! Depcrate_string_refimpl_147 {
() => {
// Module: crate::string_ref
// Provides: {"impl_147"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's KStringCowBase < 's , B > > for KStringRef < 's > { # [inline] fn from (other : & 's KStringCowBase < 's , B >) -> Self { other . as_ref () } }
};
}
