// Generated macro for impl_59 (impl)
macro_rules! Depcrate_stringimpl_59 {
() => {
// Module: crate::string
// Provides: {"impl_59"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's KStringRef < 's > > for KStringBase < B > { # [inline] fn from (other : & 's KStringRef < 's >) -> Self { other . to_owned () } }
};
}
