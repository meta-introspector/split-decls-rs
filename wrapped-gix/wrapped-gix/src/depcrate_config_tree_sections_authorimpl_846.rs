// Generated macro for impl_846 (impl)
macro_rules! Depcrate_config_tree_sections_authorimpl_846 {
() => {
// Module: crate::config::tree::sections::author
// Provides: {"impl_846"}
// Dependencies: {}
impl Author { # [doc = " The `author.name` key."] pub const NAME : keys :: Any = keys :: Any :: new ("name" , & config :: Tree :: AUTHOR) . with_fallback (& gitoxide :: Author :: NAME_FALLBACK) ; # [doc = " The `author.email` key."] pub const EMAIL : keys :: Any = keys :: Any :: new ("email" , & config :: Tree :: AUTHOR) . with_fallback (& gitoxide :: Author :: EMAIL_FALLBACK) ; }
};
}
