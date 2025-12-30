// Generated macro for impl_1038 (impl)
macro_rules! Depcrate_config_tree_sections_userimpl_1038 {
() => {
// Module: crate::config::tree::sections::user
// Provides: {"impl_1038"}
// Dependencies: {}
impl User { # [doc = " The `user.name` key"] pub const NAME : keys :: Any = keys :: Any :: new ("name" , & config :: Tree :: USER) ; # [doc = " The `user.email` key"] pub const EMAIL : keys :: Any = keys :: Any :: new ("email" , & config :: Tree :: USER) . with_fallback (& gitoxide :: User :: EMAIL_FALLBACK) ; }
};
}
