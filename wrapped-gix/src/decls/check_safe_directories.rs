macro_rules! deps {
    () => {
        Error!();
        Path!();
    };
}

macro_rules! check_safe_directories {
    () => {
        deps!();
        fn check_safe_directories (path_to_test : & std :: path :: Path , git_install_dir : Option < & std :: path :: Path > , current_dir : & std :: path :: Path , home : Option < & std :: path :: Path > , safe_dirs : & [BString] ,) -> Result < () , Error > { let mut is_safe = false ; let path_to_test = match gix_path :: realpath_opts (path_to_test , current_dir , gix_path :: realpath :: MAX_SYMLINKS) { Ok (p) => p , Err (_) => path_to_test . to_owned () , } ; for safe_dir in safe_dirs { let safe_dir = safe_dir . as_bstr () ; if safe_dir == "*" { is_safe = true ; continue ; } if safe_dir . is_empty () { is_safe = false ; continue ; } if ! is_safe { let safe_dir = match gix_config :: Path :: from (Cow :: Borrowed (safe_dir)) . interpolate (interpolate_context (git_install_dir , home)) { Ok (path) => path , Err (_) => gix_path :: from_bstr (safe_dir) , } ; if ! safe_dir . is_absolute () { gix_trace :: warn ! ("safe.directory '{safe_dir}' not absolute" , safe_dir = safe_dir . display ()) ; continue ; } if safe_dir . ends_with ("*") { let safe_dir = safe_dir . parent () . expect ("* is last component") ; if path_to_test . strip_prefix (safe_dir) . is_ok () { is_safe = true ; } } else if safe_dir == path_to_test { is_safe = true ; } } } if is_safe { Ok (()) } else { Err (Error :: UnsafeGitDir { path : path_to_test }) } }
    };
}

check_safe_directories!();