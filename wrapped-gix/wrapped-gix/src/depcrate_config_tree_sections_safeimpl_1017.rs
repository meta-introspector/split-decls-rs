// Generated macro for impl_1017 (impl)
macro_rules! Depcrate_config_tree_sections_safeimpl_1017 {
() => {
// Module: crate::config::tree::sections::safe
// Provides: {"impl_1017"}
// Dependencies: {}
impl Safe { # [doc = " Implements the directory filter to trust only global and system files, for use with `safe.directory`."] pub fn directory_filter (meta : & gix_config :: file :: Metadata) -> bool { let kind = meta . source . kind () ; kind == gix_config :: source :: Kind :: System || kind == gix_config :: source :: Kind :: Global } }
};
}
