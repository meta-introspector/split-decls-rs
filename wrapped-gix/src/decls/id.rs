macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! id {
    () => {
        deps!();
        pub (crate) fn id (git_dir : & std :: path :: Path , has_common_dir : bool) -> Option < & BStr > { if ! has_common_dir { return None ; } let candidate = gix_path :: os_str_into_bstr (git_dir . file_name () . expect ("at least one directory level")) . expect ("no illformed UTF-8") ; let maybe_worktrees = git_dir . parent () ? ; (maybe_worktrees . file_name () ? . to_str () ? == "worktrees") . then_some (candidate) }
    };
}

id!();