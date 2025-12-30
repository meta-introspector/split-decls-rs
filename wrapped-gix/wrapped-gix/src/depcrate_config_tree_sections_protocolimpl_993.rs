// Generated macro for impl_993 (impl)
macro_rules! Depcrate_config_tree_sections_protocolimpl_993 {
() => {
// Module: crate::config::tree::sections::protocol
// Provides: {"impl_993"}
// Dependencies: {}
impl Section for Protocol { fn name (& self) -> & str { "protocol" } fn keys (& self) -> & [& dyn Key] { & [& Self :: ALLOW , & Self :: VERSION] } fn sub_sections (& self) -> & [& dyn Section] { & [& Self :: NAME_PARAMETER] } }
};
}
