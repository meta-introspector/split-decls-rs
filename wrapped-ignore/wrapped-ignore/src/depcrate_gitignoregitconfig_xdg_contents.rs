// Generated macro for gitconfig_xdg_contents (function)
macro_rules! Depcrate_gitignoregitconfig_xdg_contents {
() => {
// Module: crate::gitignore
// Provides: {"gitconfig_xdg_contents"}
// Dependencies: {}
# [doc = " Returns the file contents of git's global config file, if one exists, in"] # [doc = " the user's XDG_CONFIG_HOME directory."] fn gitconfig_xdg_contents () -> Option < Vec < u8 > > { let path = std :: env :: var_os ("XDG_CONFIG_HOME") . and_then (| x | if x . is_empty () { None } else { Some (PathBuf :: from (x)) }) . or_else (| | home_dir () . map (| p | p . join (".config"))) . map (| x | x . join ("git/config")) ; let mut file = match path . and_then (| p | File :: open (p) . ok ()) { None => return None , Some (file) => BufReader :: new (file) , } ; let mut contents = vec ! [] ; file . read_to_end (& mut contents) . ok () . map (| _ | contents) }
};
}
