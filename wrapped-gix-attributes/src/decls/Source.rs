macro_rules! Source {
    () => {
        # [doc = " A list of known global sources for git attribute files in order of ascending precedence."] # [doc = ""] # [doc = " This means that values from the first variant will be returned first."] # [derive (Clone , Copy , Debug , Eq , PartialEq , Hash , Ord , PartialOrd)] pub enum Source { # [doc = " The attribute file that the installation itself ships with."] GitInstallation , # [doc = " System-wide attributes file. This is typically defined as"] # [doc = " `$(prefix)/etc/gitattributes` (where prefix is the git-installation directory)."] System , # [doc = " This is `<xdg-config-home>/git/attributes` and is git application configuration per user."] # [doc = ""] # [doc = " Note that there is no `~/.gitattributes` file."] Git , # [doc = " The configuration of the repository itself, located in `$GIT_DIR/info/attributes`."] Local , }
    };
}

Source!()