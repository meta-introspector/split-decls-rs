macro_rules! find_git_associated_windows_executable_with_fallback {
    () => {
        # [doc = " Like `find_associated_windows_executable`, but if not found, fall back to a simple filename."] pub (super) fn find_git_associated_windows_executable_with_fallback (stem : & str) -> OsString { find_git_associated_windows_executable (stem) . unwrap_or_else (| | { let mut raw_path = OsString :: from (stem) ; raw_path . push (".exe") ; raw_path }) }
    };
}

find_git_associated_windows_executable_with_fallback!()