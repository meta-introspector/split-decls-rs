// Generated macro for gitconfig_excludes_path (function)
macro_rules! Depcrate_gitignoregitconfig_excludes_path {
() => {
// Module: crate::gitignore
// Provides: {"gitconfig_excludes_path"}
// Dependencies: {}
# [doc = " Return the file path of the current environment's global gitignore file."] # [doc = ""] # [doc = " Note that the file path returned may not exist."] pub fn gitconfig_excludes_path () -> Option < PathBuf > { match gitconfig_home_contents () . and_then (| x | parse_excludes_file (& x)) { Some (path) => return Some (path) , None => { } } match gitconfig_xdg_contents () . and_then (| x | parse_excludes_file (& x)) { Some (path) => return Some (path) , None => { } } excludes_file_default () }
};
}
