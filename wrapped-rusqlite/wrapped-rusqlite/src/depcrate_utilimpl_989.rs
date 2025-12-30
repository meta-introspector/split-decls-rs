// Generated macro for impl_989 (impl)
macro_rules! Depcrate_utilimpl_989 {
() => {
// Module: crate::util
// Provides: {"impl_989"}
// Dependencies: {}
impl std :: ops :: Deref for Named < '_ > { type Target = CStr ; # [inline] fn deref (& self) -> & CStr { match self { Named :: Small (s) => s . as_cstr () , Named :: C (s) => s , } } }
};
}
