// Generated macro for excludes_file_default (function)
macro_rules! Depcrate_gitignoreexcludes_file_default {
() => {
// Module: crate::gitignore
// Provides: {"excludes_file_default"}
// Dependencies: {}
# [doc = " Returns the default file path for a global .gitignore file."] # [doc = ""] # [doc = " Specifically, this respects XDG_CONFIG_HOME."] fn excludes_file_default () -> Option < PathBuf > { std :: env :: var_os ("XDG_CONFIG_HOME") . and_then (| x | if x . is_empty () { None } else { Some (PathBuf :: from (x)) }) . or_else (| | home_dir () . map (| p | p . join (".config"))) . map (| x | x . join ("git/ignore")) }
};
}
