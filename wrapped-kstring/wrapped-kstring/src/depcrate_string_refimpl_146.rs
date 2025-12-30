// Generated macro for impl_146 (impl)
macro_rules! Depcrate_string_refimpl_146 {
() => {
// Module: crate::string_ref
// Provides: {"impl_146"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's KStringBase < B > > for KStringRef < 's > { # [inline] fn from (other : & 's KStringBase < B >) -> Self { other . as_ref () } }
};
}
