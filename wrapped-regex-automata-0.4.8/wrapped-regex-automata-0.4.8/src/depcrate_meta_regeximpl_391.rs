// Generated macro for impl_391 (impl)
macro_rules! Depcrate_meta_regeximpl_391 {
() => {
// Module: crate::meta::regex
// Provides: {"impl_391"}
// Dependencies: {}
impl Clone for Regex { fn clone (& self) -> Regex { let imp = Arc :: clone (& self . imp) ; let pool = { let strat = Arc :: clone (& imp . strat) ; let create : CachePoolFn = Box :: new (move | | strat . create_cache ()) ; Pool :: new (create) } ; Regex { imp , pool } } }
};
}
