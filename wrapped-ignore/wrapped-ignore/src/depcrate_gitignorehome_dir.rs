// Generated macro for home_dir (function)
macro_rules! Depcrate_gitignorehome_dir {
() => {
// Module: crate::gitignore
// Provides: {"home_dir"}
// Dependencies: {}
# [doc = " Returns the location of the user's home directory."] fn home_dir () -> Option < PathBuf > { # ! [allow (deprecated)] std :: env :: home_dir () }
};
}
