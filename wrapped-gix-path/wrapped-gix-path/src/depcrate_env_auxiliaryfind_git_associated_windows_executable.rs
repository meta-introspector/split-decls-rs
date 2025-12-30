// Generated macro for find_git_associated_windows_executable (function)
macro_rules! Depcrate_env_auxiliaryfind_git_associated_windows_executable {
() => {
// Module: crate::env::auxiliary
// Provides: {"find_git_associated_windows_executable"}
// Dependencies: {}
# [doc = " Obtain a path to an executable command on Windows associated with Git, if one can be found."] # [doc = ""] # [doc = " The resulting path uses only `/` separators so long as the path obtained from `git --exec-path`"] # [doc = " does, which is the case unless it is overridden by setting `GIT_EXEC_PATH` to an unusual value."] # [doc = ""] # [doc = " This is currently only used (and only heavily exercised in tests) for finding `sh.exe`. It may"] # [doc = " be used to find other executables in the future, but may need adjustment. In particular,"] # [doc = " depending on desired semantics, it should possibly also check a `cmd` directory; directories"] # [doc = " like `<platform>/bin`, for any applicable variants (such as `mingw64`); and `super::core_dir()`"] # [doc = " itself, which it could safely check even if its value is not safe for inferring other paths."] fn find_git_associated_windows_executable (stem : & str) -> Option < OsString > { let git_root = git_for_windows_root () ? ; BIN_DIR_FRAGMENTS . iter () . map (| bin_dir_fragment | { let mut raw_path = OsString :: from (git_root) ; raw_path . push ("/") ; raw_path . push (bin_dir_fragment) ; raw_path . push ("/") ; raw_path . push (stem) ; raw_path . push (".exe") ; raw_path }) . find (| raw_path | Path :: new (raw_path) . is_file ()) }
};
}
