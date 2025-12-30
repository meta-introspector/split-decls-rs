// Generated macro for find_binary_in_output2 (function)
macro_rules! Depcratefind_binary_in_output2 {
() => {
// Module: crate
// Provides: {"find_binary_in_output2"}
// Dependencies: {}
fn find_binary_in_output2 (bin_name : & str) -> Result < std :: path :: PathBuf , Box < dyn std :: error :: Error > > { let search_paths = [format ! ("../output2/wrapped-split-decls-rs/src/decls/{}/fn" , bin_name) , format ! ("../output2/wrapped-split-decls-rs/src/decls/{}" , bin_name) ,] ; for search_path in & search_paths { let path = Path :: new (search_path) ; if path . exists () { for entry in fs :: read_dir (path) ? { let entry = entry ? ; if entry . file_type () ? . is_dir () { let main_file = entry . path () . join ("main.rs") ; if main_file . exists () { return Ok (main_file) ; } } } } } Err (format ! ("Binary {} not found in output2" , bin_name) . into ()) }
};
}
