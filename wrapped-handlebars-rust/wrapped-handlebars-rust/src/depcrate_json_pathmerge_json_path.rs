// Generated macro for merge_json_path (function)
macro_rules! Depcrate_json_pathmerge_json_path {
() => {
// Module: crate::json::path
// Provides: {"merge_json_path"}
// Dependencies: {}
pub (crate) fn merge_json_path (path_stack : & mut Vec < String > , relative_path : & [PathSeg]) { for seg in relative_path { match seg { PathSeg :: Named (ref s) => { path_stack . push (s . to_owned ()) ; } PathSeg :: Ruled (Rule :: path_root) => { } PathSeg :: Ruled (Rule :: path_up) => { } _ => { } } } }
};
}
