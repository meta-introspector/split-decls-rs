// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_config_tree_sections_sshimpl_1022 {
() => {
// Module: crate::config::tree::sections::ssh
// Provides: {"impl_1022"}
// Dependencies: {}
impl Ssh { # [doc = " The `ssh.variant` key"] pub const VARIANT : Variant = Variant :: new_with_validate ("variant" , & config :: Tree :: SSH , validate :: Variant) . with_environment_override ("GIT_SSH_VARIANT") . with_deviation ("We error if a variant is chosen that we don't know, as opposed to defaulting to 'ssh'") ; }
};
}
