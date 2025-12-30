// Generated macro for split_path_dir_and_file (function)
macro_rules! Depcrate_debuginfo_line_infosplit_path_dir_and_file {
() => {
// Module: crate::debuginfo::line_info
// Provides: {"split_path_dir_and_file"}
// Dependencies: {}
fn split_path_dir_and_file (path : & Path) -> (& Path , & OsStr) { let mut iter = path . components () ; let file_name = match iter . next_back () { Some (Component :: Normal (p)) => p , component => { panic ! ("Path component {:?} of path {} is an invalid filename" , component , path . display ()) ; } } ; let parent = iter . as_path () ; (parent , file_name) }
};
}
