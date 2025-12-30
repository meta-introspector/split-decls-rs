// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_config_tree_sections_userimpl_1039 {
() => {
// Module: crate::config::tree::sections::user
// Provides: {"impl_1039"}
// Dependencies: {}
impl Section for User { fn name (& self) -> & str { "user" } fn keys (& self) -> & [& dyn Key] { & [& Self :: NAME , & Self :: EMAIL] } }
};
}
