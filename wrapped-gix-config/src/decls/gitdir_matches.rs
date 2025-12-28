macro_rules! deps {
    () => {
        Error!();
        Options!();
    };
}

macro_rules! gitdir_matches {
    () => {
        deps!();
        fn gitdir_matches (condition_path : & BStr , target_config_path : Option < & Path > , Options { conditional : conditional :: Context { git_dir , .. } , interpolate : context , err_on_interpolation_failure , err_on_missing_config_path , .. } : Options < '_ > , wildmatch_mode : gix_glob :: wildmatch :: Mode ,) -> Result < bool , Error > { if ! err_on_interpolation_failure && git_dir . is_none () { return Ok (false) ; } let git_dir = gix_path :: to_unix_separators_on_windows (gix_path :: into_bstr (git_dir . ok_or (Error :: MissingGitDir) ?)) ; let mut pattern_path : Cow < '_ , _ > = { let path = match check_interpolation_result (err_on_interpolation_failure , crate :: Path :: from (Cow :: Borrowed (condition_path)) . interpolate (context) ,) ? { Some (p) => p , None => return Ok (false) , } ; gix_path :: into_bstr (path) . into_owned () . into () } ; if pattern_path != condition_path { pattern_path = gix_path :: to_unix_separators_on_windows (pattern_path) ; } if let Some (relative_pattern_path) = pattern_path . strip_prefix (b"./") { if ! err_on_missing_config_path && target_config_path . is_none () { return Ok (false) ; } let parent_dir = target_config_path . ok_or (Error :: MissingConfigPath) ? . parent () . expect ("config path can never be /") ; let mut joined_path = gix_path :: to_unix_separators_on_windows (gix_path :: into_bstr (parent_dir)) . into_owned () ; joined_path . push (b'/') ; joined_path . extend_from_slice (relative_pattern_path) ; pattern_path = joined_path . into () ; } if pattern_path . iter () . next () != Some (& (std :: path :: MAIN_SEPARATOR as u8)) && ! gix_path :: from_bstr (pattern_path . clone ()) . is_absolute () { let mut prefixed = pattern_path . into_owned () ; prefixed . insert_str (0 , "**/") ; pattern_path = prefixed . into () ; } if pattern_path . ends_with (b"/") { let mut suffixed = pattern_path . into_owned () ; suffixed . push_str ("**") ; pattern_path = suffixed . into () ; } let match_mode = gix_glob :: wildmatch :: Mode :: NO_MATCH_SLASH_LITERAL | wildmatch_mode ; let is_match = gix_glob :: wildmatch (pattern_path . as_bstr () , git_dir . as_bstr () , match_mode) ; if is_match { return Ok (true) ; } let expanded_git_dir = gix_path :: into_bstr (gix_path :: realpath (gix_path :: from_byte_slice (& git_dir)) ?) ; Ok (gix_glob :: wildmatch (pattern_path . as_bstr () , expanded_git_dir . as_bstr () , match_mode ,)) }
    };
}

gitdir_matches!()