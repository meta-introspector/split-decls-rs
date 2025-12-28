macro_rules! parse_ceiling_dirs {
    () => {
        # [doc = " Parse a byte-string of `:`-separated paths into `Vec<PathBuf>`."] # [doc = " On Windows, paths are separated by `;`."] # [doc = " Non-absolute paths are discarded."] # [doc = " To match git, all paths are normalized, until an empty path is encountered."] pub (crate) fn parse_ceiling_dirs (ceiling_dirs : & OsStr) -> Vec < PathBuf > { let mut should_normalize = true ; let mut out = Vec :: new () ; for ceiling_dir in std :: env :: split_paths (ceiling_dirs) { if ceiling_dir . as_os_str () . is_empty () { should_normalize = false ; continue ; } if ceiling_dir . is_relative () { continue ; } let mut dir = ceiling_dir ; if should_normalize { if let Ok (normalized) = gix_path :: realpath (& dir) { dir = normalized ; } } out . push (dir) ; } out }
    };
}

parse_ceiling_dirs!();