// Generated macro for expand_tilde (function)
macro_rules! Depcrate_gitignoreexpand_tilde {
() => {
// Module: crate::gitignore
// Provides: {"expand_tilde"}
// Dependencies: {}
# [doc = " Expands ~ in file paths to the value of $HOME."] fn expand_tilde (path : & str) -> String { let home = match home_dir () { None => return path . to_string () , Some (home) => home . to_string_lossy () . into_owned () , } ; path . replace ("~" , & home) }
};
}
