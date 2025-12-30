// Generated macro for impl_58 (impl)
macro_rules! Depcrate_stringimpl_58 {
() => {
// Module: crate::string
// Provides: {"impl_58"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < KStringRef < 's > > for KStringBase < B > { # [inline] fn from (other : KStringRef < 's >) -> Self { other . to_owned () } }
};
}
