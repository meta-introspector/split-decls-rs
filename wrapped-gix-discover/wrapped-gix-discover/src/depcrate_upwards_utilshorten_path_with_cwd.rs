// Generated macro for shorten_path_with_cwd (function)
macro_rules! Depcrate_upwards_utilshorten_path_with_cwd {
() => {
// Module: crate::upwards::util
// Provides: {"shorten_path_with_cwd"}
// Dependencies: {}
pub (crate) fn shorten_path_with_cwd (cursor : PathBuf , cwd : & Path) -> PathBuf { fn comp_len (c : std :: path :: Component < '_ >) -> usize { use std :: path :: Component :: * ; match c { Prefix (p) => p . as_os_str () . len () , CurDir => 1 , ParentDir => 2 , Normal (p) => p . len () , RootDir => 1 , } } debug_assert_eq ! (cursor . file_name () . and_then (std :: ffi :: OsStr :: to_str) , Some (DOT_GIT_DIR)) ; let parent = cursor . parent () . expect (".git appended") ; cwd . strip_prefix (parent) . ok () . and_then (| path_relative_to_cwd | { let relative_path_components = path_relative_to_cwd . components () . count () ; let current_component_len = cursor . components () . map (comp_len) . sum :: < usize > () ; (relative_path_components * ".." . len () < current_component_len) . then (| | { std :: iter :: repeat_n (".." , relative_path_components) . chain (Some (DOT_GIT_DIR)) . collect () }) }) . unwrap_or (cursor) }
};
}
