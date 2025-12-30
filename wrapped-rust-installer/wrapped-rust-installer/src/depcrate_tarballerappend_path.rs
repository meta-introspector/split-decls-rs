// Generated macro for append_path (function)
macro_rules! Depcrate_tarballerappend_path {
() => {
// Module: crate::tarballer
// Provides: {"append_path"}
// Dependencies: {}
fn append_path < W : Write > (builder : & mut Builder < W > , src : & Path , path : & String , override_file_mtime : u64 ,) -> Result < () > { let stat = symlink_metadata (src) ? ; let mut header = Header :: new_gnu () ; header . set_metadata_in_mode (& stat , HeaderMode :: Deterministic) ; header . set_mtime (override_file_mtime) ; if stat . file_type () . is_symlink () { let link = read_link (src) ? ; builder . append_link (& mut header , path , & link) ? ; } else { if cfg ! (windows) { const EXECUTABLES : [& str ; 4] = ["exe" , "dll" , "py" , "sh"] ; if let Some (ext) = src . extension () . and_then (| s | s . to_str ()) { if EXECUTABLES . contains (& ext) { let mode = header . mode () ? ; header . set_mode (mode | 0o111) ; } } } let file = open_file (src) ? ; builder . append_data (& mut header , path , & file) ? ; } Ok (()) }
};
}
