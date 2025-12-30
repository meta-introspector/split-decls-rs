// Generated macro for impl_65 (impl)
macro_rules! Depcrate_stringimpl_65 {
() => {
// Module: crate::string
// Provides: {"impl_65"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's crate :: backend :: BoxedStr > for KStringBase < B > { # [inline] fn from (other : & 's crate :: backend :: BoxedStr) -> Self { Self :: from_ref (other) } }
};
}
