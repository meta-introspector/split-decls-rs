// Generated macro for impl_992 (impl)
macro_rules! Depcrate_config_tree_sections_protocolimpl_992 {
() => {
// Module: crate::config::tree::sections::protocol
// Provides: {"impl_992"}
// Dependencies: {}
impl Section for NameParameter { fn name (& self) -> & str { "<name>" } fn keys (& self) -> & [& dyn Key] { & [& Self :: ALLOW] } fn parent (& self) -> Option < & dyn Section > { Some (& config :: Tree :: PROTOCOL) } }
};
}
