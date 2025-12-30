// Generated macro for get_target_dir (function)
macro_rules! Depcrate_utilget_target_dir {
() => {
// Module: crate::util
// Provides: {"get_target_dir"}
// Dependencies: {}
# [doc = " Get the target directory for miri output."] # [doc = ""] # [doc = " Either in an argument passed-in, or from cargo metadata."] pub fn get_target_dir (meta : & Metadata) -> PathBuf { let mut output = match get_arg_flag_value ("--target-dir") { Some (dir) => PathBuf :: from (dir) , None => meta . target_directory . clone () . into_std_path_buf () , } ; output . push ("miri") ; output }
};
}
