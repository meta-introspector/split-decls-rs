// Generated macro for config_to_base_path (function)
macro_rules! Depcrate_env_gitconfig_to_base_path {
() => {
// Module: crate::env::git
// Provides: {"config_to_base_path"}
// Dependencies: {}
# [doc = " Given `config_path` as obtained from `install_config_path()`, return the path of the git installation base."] pub (super) fn config_to_base_path (config_path : & Path) -> & Path { config_path . parent () . expect ("config file paths always have a file name to pop") }
};
}
