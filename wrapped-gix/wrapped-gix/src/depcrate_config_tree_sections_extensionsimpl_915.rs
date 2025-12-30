// Generated macro for impl_915 (impl)
macro_rules! Depcrate_config_tree_sections_extensionsimpl_915 {
() => {
// Module: crate::config::tree::sections::extensions
// Provides: {"impl_915"}
// Dependencies: {}
impl Extensions { # [doc = " The `extensions.worktreeConfig` key."] pub const WORKTREE_CONFIG : keys :: Boolean = keys :: Boolean :: new_boolean ("worktreeConfig" , & config :: Tree :: EXTENSIONS) ; # [doc = " The `extensions.objectFormat` key."] pub const OBJECT_FORMAT : ObjectFormat = ObjectFormat :: new_with_validate ("objectFormat" , & config :: Tree :: EXTENSIONS , validate :: ObjectFormat) . with_note ("Support for SHA256 is prepared but not fully implemented yet. For now we abort when encountered" ,) ; }
};
}
