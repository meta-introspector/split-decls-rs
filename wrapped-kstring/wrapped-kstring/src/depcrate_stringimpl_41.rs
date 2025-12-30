// Generated macro for impl_41 (impl)
macro_rules! Depcrate_stringimpl_41 {
() => {
// Module: crate::string
// Provides: {"impl_41"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > std :: ops :: Deref for KStringBase < B > { type Target = str ; # [inline] fn deref (& self) -> & str { self . as_str () } }
};
}
