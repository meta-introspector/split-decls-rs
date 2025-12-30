// Generated macro for impl_199 (impl)
macro_rules! Depcrate_collections_stringimpl_199 {
() => {
// Module: crate::collections::string
// Provides: {"impl_199"}
// Dependencies: {}
impl < 'bump > Clone for String < 'bump > { fn clone (& self) -> Self { String { vec : self . vec . clone () , } } fn clone_from (& mut self , source : & Self) { self . vec . clone_from (& source . vec) ; } }
};
}
