// Generated macro for impl_1158 (impl)
macro_rules! Depcrate_core_config_toml_rustimpl_1158 {
() => {
// Module: crate::core::config::toml::rust
// Provides: {"impl_1158"}
// Dependencies: {}
impl RustOptimize { pub (crate) fn is_release (& self) -> bool { match & self { RustOptimize :: Bool (true) | RustOptimize :: String (_) => true , RustOptimize :: Int (i) => * i > 0 , RustOptimize :: Bool (false) => false , } } pub (crate) fn get_opt_level (& self) -> Option < String > { match & self { RustOptimize :: String (s) => Some (s . clone ()) , RustOptimize :: Int (i) => Some (i . to_string ()) , RustOptimize :: Bool (_) => None , } } }
};
}
