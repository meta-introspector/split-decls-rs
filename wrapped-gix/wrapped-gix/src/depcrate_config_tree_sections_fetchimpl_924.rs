// Generated macro for impl_924 (impl)
macro_rules! Depcrate_config_tree_sections_fetchimpl_924 {
() => {
// Module: crate::config::tree::sections::fetch
// Provides: {"impl_924"}
// Dependencies: {}
impl Section for Fetch { fn name (& self) -> & str { "fetch" } fn keys (& self) -> & [& dyn Key] { & [& Self :: NEGOTIATION_ALGORITHM , # [cfg (feature = "attributes")] & Self :: RECURSE_SUBMODULES ,] } }
};
}
