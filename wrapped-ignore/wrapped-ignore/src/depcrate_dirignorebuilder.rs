// Generated macro for IgnoreBuilder (struct)
macro_rules! Depcrate_dirIgnoreBuilder {
() => {
// Module: crate::dir
// Provides: {"IgnoreBuilder"}
// Dependencies: {}
# [doc = " A builder for creating an Ignore matcher."] # [derive (Clone , Debug)] pub (crate) struct IgnoreBuilder { # [doc = " The root directory path for this ignore matcher."] dir : PathBuf , # [doc = " An override matcher (default is empty)."] overrides : Arc < Override > , # [doc = " A type matcher (default is empty)."] types : Arc < Types > , # [doc = " Explicit global ignore matchers."] explicit_ignores : Vec < Gitignore > , # [doc = " Ignore files in addition to .ignore."] custom_ignore_filenames : Vec < OsString > , # [doc = " The directory that gitignores should be interpreted relative to."] # [doc = ""] # [doc = " Usually this is the directory containing the gitignore file. But in"] # [doc = " some cases, like for global gitignores or for gitignores specified"] # [doc = " explicitly, this should generally be set to the current working"] # [doc = " directory. This is only used for global gitignores or \"explicit\""] # [doc = " gitignores."] # [doc = ""] # [doc = " When `None`, global gitignores are ignored."] global_gitignores_relative_to : Option < PathBuf > , # [doc = " Ignore config."] opts : IgnoreOptions , }
};
}
