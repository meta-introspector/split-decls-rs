macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! bare {
    () => {
        deps!();
        # [doc = " Returns true if the given `git_dir` seems to be a bare repository."] # [doc = ""] # [doc = " Please note that repositories without an index generally _look_ bare, even though they might also be uninitialized."] pub fn bare (git_dir_candidate : & Path) -> bool { ! (git_dir_candidate . join ("index") . exists () || (git_dir_candidate . file_name () == Some (OsStr :: new (DOT_GIT_DIR)))) }
    };
}

bare!()