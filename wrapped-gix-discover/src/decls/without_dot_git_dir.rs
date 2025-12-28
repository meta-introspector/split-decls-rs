macro_rules! without_dot_git_dir {
    () => {
        # [doc = " Conditionally pop a trailing `.git` dir if present."] pub fn without_dot_git_dir (mut path : PathBuf) -> PathBuf { if path . file_name () . and_then (std :: ffi :: OsStr :: to_str) == Some (DOT_GIT_DIR) { path . pop () ; } path }
    };
}

without_dot_git_dir!()