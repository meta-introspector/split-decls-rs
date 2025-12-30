// Generated macro for remap_paths (function)
macro_rules! Depcrate_core_builderremap_paths {
() => {
// Module: crate::core::builder
// Provides: {"remap_paths"}
// Dependencies: {}
fn remap_paths (paths : & mut Vec < PathBuf >) { let mut remove = vec ! [] ; let mut add = vec ! [] ; for (i , path) in paths . iter () . enumerate () . filter_map (| (i , path) | path . to_str () . map (| s | (i , s))) { for & (search , replace) in PATH_REMAP { if path . trim_matches (std :: path :: is_separator) == search { remove . push (i) ; add . extend (replace . iter () . map (PathBuf :: from)) ; break ; } } } remove . sort () ; remove . dedup () ; for idx in remove . into_iter () . rev () { paths . remove (idx) ; } paths . append (& mut add) ; }
};
}
