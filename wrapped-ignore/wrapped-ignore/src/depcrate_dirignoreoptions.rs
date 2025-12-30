// Generated macro for IgnoreOptions (struct)
macro_rules! Depcrate_dirIgnoreOptions {
() => {
// Module: crate::dir
// Provides: {"IgnoreOptions"}
// Dependencies: {}
# [doc = " Options for the ignore matcher, shared between the matcher itself and the"] # [doc = " builder."] # [derive (Clone , Copy , Debug)] struct IgnoreOptions { # [doc = " Whether to ignore hidden file paths or not."] hidden : bool , # [doc = " Whether to read .ignore files."] ignore : bool , # [doc = " Whether to respect any ignore files in parent directories."] parents : bool , # [doc = " Whether to read git's global gitignore file."] git_global : bool , # [doc = " Whether to read .gitignore files."] git_ignore : bool , # [doc = " Whether to read .git/info/exclude files."] git_exclude : bool , # [doc = " Whether to ignore files case insensitively"] ignore_case_insensitive : bool , # [doc = " Whether a git repository must be present in order to apply any"] # [doc = " git-related ignore rules."] require_git : bool , }
};
}
