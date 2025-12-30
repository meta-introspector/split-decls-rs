// Generated macro for git_options (function)
macro_rules! Depcrategit_options {
() => {
// Module: crate
// Provides: {"git_options"}
// Dependencies: {}
# [doc = " Create a Git cache populated with the arguments that are going to be"] # [doc = " listed before they’re actually listed, if the options demand it."] fn git_options (options : & Options , args : & [& OsStr]) -> Option < GitCache > { if options . should_scan_for_git () { Some (args . iter () . map (PathBuf :: from) . collect ()) } else { None } }
};
}
