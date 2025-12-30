// Generated macro for get_files_in_dir (function)
macro_rules! Depcrateget_files_in_dir {
() => {
// Module: crate
// Provides: {"get_files_in_dir"}
// Dependencies: {}
# [cfg (feature = "git")] fn get_files_in_dir (paths : & mut Vec < PathBuf > , path : PathBuf) { let temp_paths = if path . is_dir () { match path . read_dir () { Err (_) => { vec ! [path] } Ok (d) => d . filter_map (| entry | entry . ok () . map (| e | e . path ())) . collect :: < Vec < PathBuf > > () , } } else { vec ! [path] } ; paths . extend (temp_paths) ; }
};
}
