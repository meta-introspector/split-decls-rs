// Generated macro for create_dirs (function)
macro_rules! Depcrate_inputcreate_dirs {
() => {
// Module: crate::input
// Provides: {"create_dirs"}
// Dependencies: {}
# [doc = " Create necessary directories to run the lintcheck tool."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function panics if creating one of the dirs fails."] fn create_dirs (krate_download_dir : & Path , extract_dir : & Path) { fs :: create_dir (format ! ("{}/lintcheck/" , target_dir ())) . unwrap_or_else (| err | { assert_eq ! (err . kind () , ErrorKind :: AlreadyExists , "cannot create lintcheck target dir") ; }) ; fs :: create_dir_all (krate_download_dir) . unwrap_or_else (| err | { assert_ne ! (err . kind () , ErrorKind :: AlreadyExists) ; }) ; fs :: create_dir (extract_dir) . unwrap_or_else (| err | { assert_eq ! (err . kind () , ErrorKind :: AlreadyExists , "cannot create crate extraction dir") ; }) ; }
};
}
