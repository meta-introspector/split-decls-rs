// Generated macro for impl_971 (impl)
macro_rules! Depcrate_config_tree_sections_mergeimpl_971 {
() => {
// Module: crate::config::tree::sections::merge
// Provides: {"impl_971"}
// Dependencies: {}
impl Section for Merge { fn name (& self) -> & str { "merge" } fn keys (& self) -> & [& dyn Key] { & [& Self :: RENAME_LIMIT , # [cfg (feature = "merge")] & Self :: RENAMES , & Self :: RENORMALIZE , & Self :: DEFAULT , & Self :: DRIVER_NAME , & Self :: DRIVER_COMMAND , & Self :: DRIVER_RECURSIVE , # [cfg (feature = "merge")] & Self :: CONFLICT_STYLE ,] } }
};
}
