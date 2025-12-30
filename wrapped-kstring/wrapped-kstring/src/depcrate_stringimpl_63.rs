// Generated macro for impl_63 (impl)
macro_rules! Depcrate_stringimpl_63 {
() => {
// Module: crate::string
// Provides: {"impl_63"}
// Dependencies: {}
impl < 's , B : crate :: backend :: HeapStr > From < & 's StdString > for KStringBase < B > { # [inline] fn from (other : & 's StdString) -> Self { Self :: from_ref (other) } }
};
}
