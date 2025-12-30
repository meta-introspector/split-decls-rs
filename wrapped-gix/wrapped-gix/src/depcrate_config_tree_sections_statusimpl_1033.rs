// Generated macro for impl_1033 (impl)
macro_rules! Depcrate_config_tree_sections_statusimpl_1033 {
() => {
// Module: crate::config::tree::sections::status
// Provides: {"impl_1033"}
// Dependencies: {}
impl Section for Status { fn name (& self) -> & str { "status" } fn keys (& self) -> & [& dyn Key] { & [& Self :: SHOW_UNTRACKED_FILES , & Self :: RENAMES , & Self :: RENAME_LIMIT] } }
};
}
