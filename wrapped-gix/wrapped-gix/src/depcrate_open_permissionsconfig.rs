// Generated macro for Config (struct)
macro_rules! Depcrate_open_permissionsConfig {
() => {
// Module: crate::open::permissions
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Configure from which sources git configuration may be loaded."] # [doc = ""] # [doc = " Note that configuration from inside of the repository is always loaded as it's definitely required for correctness."] # [derive (Copy , Clone , Ord , PartialOrd , PartialEq , Eq , Debug , Hash)] pub struct Config { # [doc = " The git binary may come with configuration as part of its configuration, and if this is true (default false)"] # [doc = " we will load the configuration of the git binary, if present and not a duplicate of the ones below."] # [doc = ""] # [doc = " It's disabled by default as it may involve executing the git binary once per execution of the application."] pub git_binary : bool , # [doc = " Whether to use the system configuration."] # [doc = " This is defined as `$(prefix)/etc/gitconfig` on unix."] pub system : bool , # [doc = " Whether to use the git application configuration."] # [doc = ""] # [doc = " A platform defined location for where a user's git application configuration should be located."] # [doc = " If `$XDG_CONFIG_HOME` is not set or empty, `$HOME/.config/git/config` will be used"] # [doc = " on unix."] pub git : bool , # [doc = " Whether to use the user configuration."] # [doc = " This is usually `~/.gitconfig` on unix."] pub user : bool , # [doc = " Whether to use the configuration from environment variables."] pub env : bool , # [doc = " Whether to follow include files are encountered in loaded configuration,"] # [doc = " via `include` and `includeIf` sections."] pub includes : bool , }
};
}
