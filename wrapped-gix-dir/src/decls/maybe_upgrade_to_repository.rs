macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! maybe_upgrade_to_repository {
    () => {
        deps!();
        pub fn maybe_upgrade_to_repository (current_kind : Option < entry :: Kind > , find_harder : bool , recurse_repositories : bool , path : & mut PathBuf , current_dir : & Path , git_dir_realpath : & Path ,) -> Option < entry :: Kind > { if recurse_repositories { return current_kind ; } if find_harder { let mut is_nested_repo = gix_discover :: is_git (path) . is_ok () ; if is_nested_repo { let git_dir_is_our_own = gix_path :: realpath_opts (path , current_dir , gix_path :: realpath :: MAX_SYMLINKS) . ok () . is_some_and (| realpath_candidate | realpath_candidate == git_dir_realpath) ; is_nested_repo = ! git_dir_is_our_own ; } if is_nested_repo { return Some (entry :: Kind :: Repository) ; } } path . push (gix_discover :: DOT_GIT_DIR) ; let mut is_nested_nonbare_repo = gix_discover :: is_git (path) . is_ok () ; if is_nested_nonbare_repo { let git_dir_is_our_own = gix_path :: realpath_opts (path , current_dir , gix_path :: realpath :: MAX_SYMLINKS) . ok () . is_some_and (| realpath_candidate | realpath_candidate == git_dir_realpath) ; is_nested_nonbare_repo = ! git_dir_is_our_own ; } path . pop () ; if is_nested_nonbare_repo { Some (entry :: Kind :: Repository) } else { current_kind } }
    };
}

maybe_upgrade_to_repository!()