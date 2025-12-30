// Generated macro for make_new_path (function)
macro_rules! Depcrate_utilmake_new_path {
() => {
// Module: crate::util
// Provides: {"make_new_path"}
// Dependencies: {}
pub fn make_new_path (path : & str) -> String { assert ! (cfg ! (windows)) ; match env :: var (lib_path_env_var ()) { Ok (curr) => format ! ("{}{}{}" , path , path_div () , curr) , Err (..) => path . to_owned () , } }
};
}
