// Generated macro for impl_187 (impl)
macro_rules! Depcrate_stringimpl_187 {
() => {
// Module: crate::string
// Provides: {"impl_187"}
// Dependencies: {}
impl Clone for Regex { fn clone (& self) -> Regex { let pikevm = Arc :: clone (& self . pikevm) ; let pool = { let pikevm = Arc :: clone (& self . pikevm) ; let create = Box :: new (move | | Cache :: new (& pikevm)) ; CachePool :: new (create) } ; Regex { pikevm , pool } } }
};
}
