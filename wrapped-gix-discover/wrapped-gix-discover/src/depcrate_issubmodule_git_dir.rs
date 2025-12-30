// Generated macro for submodule_git_dir (function)
macro_rules! Depcrate_issubmodule_git_dir {
() => {
// Module: crate::is
// Provides: {"submodule_git_dir"}
// Dependencies: {}
# [doc = " Returns true if `git_dir` is located within a `.git/modules` directory, indicating it's a submodule clone."] pub fn submodule_git_dir (git_dir : & Path) -> bool { let mut last_comp = None ; git_dir . file_name () != Some (OsStr :: new (DOT_GIT_DIR)) && git_dir . components () . rev () . skip (1) . any (| c | { if c . as_os_str () == OsStr :: new (DOT_GIT_DIR) { true } else { last_comp = Some (c . as_os_str ()) ; false } }) && last_comp == Some (OsStr :: new (MODULES)) }
};
}
