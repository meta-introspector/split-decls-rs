// Generated macro for Attributes (struct)
macro_rules! Depcrate_open_permissionsAttributes {
() => {
// Module: crate::open::permissions
// Provides: {"Attributes"}
// Dependencies: {}
# [doc = " Configure from which `gitattribute` files may be loaded."] # [doc = ""] # [doc = " Note that `.gitattribute` files from within the repository are always loaded."] # [derive (Copy , Clone , Ord , PartialOrd , PartialEq , Eq , Debug , Hash)] pub struct Attributes { # [doc = " The git binary may come with attribute configuration in its installation directory, and if this is true (default false)"] # [doc = " we will load the configuration of the git binary."] # [doc = ""] # [doc = " It's disabled by default as it involves executing the git binary once per execution of the application."] pub git_binary : bool , # [doc = " Whether to use the system configuration."] # [doc = " This is typically defined as `$(prefix)/etc/gitconfig`."] pub system : bool , # [doc = " Whether to use the git application configuration."] # [doc = ""] # [doc = " A platform defined location for where a user's git application configuration should be located."] # [doc = " If `$XDG_CONFIG_HOME` is not set or empty, `$HOME/.config/git/attributes` will be used"] # [doc = " on unix."] pub git : bool , }
};
}
