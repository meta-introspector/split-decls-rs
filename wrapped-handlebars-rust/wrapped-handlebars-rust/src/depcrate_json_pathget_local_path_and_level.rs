// Generated macro for get_local_path_and_level (function)
macro_rules! Depcrate_json_pathget_local_path_and_level {
() => {
// Module: crate::json::path
// Provides: {"get_local_path_and_level"}
// Dependencies: {}
fn get_local_path_and_level (paths : & [PathSeg]) -> Option < (usize , String) > { paths . first () . and_then (| seg | { if seg == & PathSeg :: Ruled (Rule :: path_local) { let mut level = 0 ; while paths . get (level + 1) ? == & PathSeg :: Ruled (Rule :: path_up) { level += 1 ; } if let Some (PathSeg :: Named (name)) = paths . get (level + 1) { Some ((level , name . clone ())) } else { None } } else { None } }) }
};
}
