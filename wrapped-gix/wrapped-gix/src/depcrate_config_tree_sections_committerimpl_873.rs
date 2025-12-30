// Generated macro for impl_873 (impl)
macro_rules! Depcrate_config_tree_sections_committerimpl_873 {
() => {
// Module: crate::config::tree::sections::committer
// Provides: {"impl_873"}
// Dependencies: {}
impl Committer { # [doc = " The `committer.name` key."] pub const NAME : keys :: Any = keys :: Any :: new ("name" , & config :: Tree :: COMMITTER) . with_fallback (& gitoxide :: Committer :: NAME_FALLBACK) ; # [doc = " The `committer.email` key."] pub const EMAIL : keys :: Any = keys :: Any :: new ("email" , & config :: Tree :: COMMITTER) . with_fallback (& gitoxide :: Committer :: EMAIL_FALLBACK) ; }
};
}
