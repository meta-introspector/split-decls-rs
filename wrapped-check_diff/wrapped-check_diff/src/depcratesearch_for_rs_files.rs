// Generated macro for search_for_rs_files (function)
macro_rules! Depcratesearch_for_rs_files {
() => {
// Module: crate
// Provides: {"search_for_rs_files"}
// Dependencies: {}
# [doc = " Searches for rust files in the particular path and returns an iterator to them."] pub fn search_for_rs_files (repo : & Path) -> impl Iterator < Item = PathBuf > { return WalkDir :: new (repo) . into_iter () . filter_map (| e | match e . ok () { Some (entry) => { let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { return Some (entry . into_path ()) ; } return None ; } None => None , }) ; }
};
}
